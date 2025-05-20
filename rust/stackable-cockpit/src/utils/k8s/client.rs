use std::{collections::BTreeMap, string::FromUtf8Error};

use k8s_openapi::api::{
    apps::v1::{Deployment, StatefulSet},
    core::v1::{Endpoints, Namespace, Node, Secret, Service},
};
use kube::{
    Api, Discovery, ResourceExt,
    api::{ListParams, Patch, PatchParams, PostParams},
    core::{DynamicObject, GroupVersionKind, ObjectList, ObjectMeta, TypeMeta},
    discovery::{ApiCapabilities, ApiResource, Scope},
};
use serde::Deserialize;
use snafu::{OptionExt, ResultExt, Snafu};
use stackable_operator::{commons::listener::Listener, kvp::Labels};
use tokio::sync::RwLock;
use tracing::{info, instrument, Span};
use tracing_indicatif::span_ext::IndicatifSpanExt as _;

#[cfg(doc)]
use crate::utils::k8s::ListParamsExt;
use crate::{
    platform::{cluster, credentials::Credentials},
    utils::k8s::ByteStringExt,
};

pub type ListResult<T, E = Error> = Result<ObjectList<T>, E>;
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("failed to create Kubernetes client"))]
    KubeClientCreate { source: kube::error::Error },

    #[snafu(display("failed to fetch data from Kubernetes API"))]
    KubeClientFetch { source: kube::error::Error },

    #[snafu(display("failed to patch/create Kubernetes object"))]
    KubeClientPatch { source: kube::error::Error },

    #[snafu(display("failed to deserialize YAML data"))]
    DeserializeYaml { source: serde_yaml::Error },

    #[snafu(display("failed to run GVK discovery"))]
    GVKDiscoveryRun { source: kube::error::Error },

    #[snafu(display("GVK {gvk:?} is not known"))]
    GVKUnkown { gvk: GroupVersionKind },

    #[snafu(display("failed to deploy manifest because type of object {object:?} is not set"))]
    ObjectType { object: DynamicObject },

    #[snafu(display("failed to deploy manifest because GVK {group}/{kind}@{version} cannot be resolved",
        group = gvk.group,
        version = gvk.version,
        kind = gvk.kind
    ))]
    DiscoveryResolve { gvk: GroupVersionKind },

    #[snafu(display("failed to convert byte string into UTF-8 string"))]
    ByteStringConvert { source: FromUtf8Error },

    #[snafu(display("missing namespace for service '{service}'"))]
    MissingServiceNamespace { service: String },

    #[snafu(display("failed to retrieve cluster information"))]
    ClusterInformation { source: cluster::Error },

    #[snafu(display("invalid or empty secret data in '{secret_name}'"))]
    InvalidSecretData { secret_name: String },

    #[snafu(display("no username key in credentials secret '{secret_name}'"))]
    NoUsernameKey { secret_name: String },

    #[snafu(display("no password key in credentials secret '{secret_name}'"))]
    NoPasswordKey { secret_name: String },
}

pub struct Client {
    client: kube::Client,

    // Choosing an [`RwLock`] here, as their can be many reads in parallel, but running a discovery is very rare
    discovery: RwLock<Discovery>,
}

impl Client {
    /// Tries to create a new default Kubernetes client and immediately runs
    /// a discovery.
    pub async fn new() -> Result<Self> {
        let client = kube::Client::try_default()
            .await
            .context(KubeClientCreateSnafu)?;

        let discovery = RwLock::new(Self::run_discovery(client.clone()).await?);

        Ok(Self { client, discovery })
    }

    /// Deploys manifests defined the in raw `manifests` YAML string. This
    /// method will fail if it is unable to parse the manifests, unable to
    /// resolve GVKs or unable to patch the dynamic objects.
    #[instrument(skip_all, fields(indicatif.pb_show = true))]
    pub async fn deploy_manifests(
        &self,
        manifests: &str,
        namespace: &str,
        labels: Labels,
    ) -> Result<()> {
        Span::current().pb_set_message("Installing YAML manifest");

        // TODO (Techassi): Impl IntoIterator for Labels
        let labels: BTreeMap<String, String> = labels.into();

        for manifest in serde_yaml::Deserializer::from_str(manifests) {
            let mut object = DynamicObject::deserialize(manifest).context(DeserializeYamlSnafu)?;

            // Add our own labels to the object
            object.labels_mut().extend(labels.clone());

            let object_type = object.types.as_ref().ok_or(
                ObjectTypeSnafu {
                    object: object.clone(),
                }
                .build(),
            )?;

            let gvk = Self::gvk_of_typemeta(object_type);
            let (resource, capabilities) = self
                .resolve_gvk(&gvk)
                .await?
                .context(GVKUnkownSnafu { gvk })?;

            let api: Api<DynamicObject> = match capabilities.scope {
                Scope::Cluster => {
                    object.metadata.namespace = None;
                    Api::all_with(self.client.clone(), &resource)
                }
                Scope::Namespaced => {
                    Api::namespaced_with(self.client.clone(), namespace, &resource)
                }
            };

            api.patch(
                &object.name_any(),
                &PatchParams::apply("stackablectl"),
                &Patch::Apply(object),
            )
            .await
            .context(KubeClientPatchSnafu)?;
        }

        Ok(())
    }

    /// Lists objects by looking up a GVK via the discovery. It returns an
    /// optional list of dynamic objects. The method returns [`Ok(None)`]
    /// if the client was unable to resolve the GVK. An error is returned
    /// when the client failed to list the objects.
    pub async fn list_objects(
        &self,
        gvk: &GroupVersionKind,
        namespace: Option<&str>,
    ) -> Result<Option<ObjectList<DynamicObject>>, Error> {
        let object_api_resource = match self.resolve_gvk(gvk).await {
            Ok(Some((object_api_resource, _))) => object_api_resource,
            _ => {
                return Ok(None);
            }
        };

        let object_api: Api<DynamicObject> = match namespace {
            Some(namespace) => {
                Api::namespaced_with(self.client.clone(), namespace, &object_api_resource)
            }
            None => Api::all_with(self.client.clone(), &object_api_resource),
        };

        let objects = object_api
            .list(&ListParams::default())
            .await
            .context(KubeClientFetchSnafu)?;

        Ok(Some(objects))
    }

    pub async fn get_namespaced_object(
        &self,
        namespace: &str,
        object_name: &str,
        gvk: &GroupVersionKind,
    ) -> Result<Option<DynamicObject>, Error> {
        let object_api_resource = match self.resolve_gvk(gvk).await {
            Ok(Some((object_api_resource, _))) => object_api_resource,
            _ => {
                return Ok(None);
            }
        };

        let api = Api::namespaced_with(self.client.clone(), namespace, &object_api_resource);
        Ok(Some(
            api.get(object_name).await.context(KubeClientFetchSnafu)?,
        ))
    }

    /// Lists [`Service`]s by matching labels. The Services can be matched by
    /// the product labels. [`ListParamsExt`] provides a utility function to
    /// create [`ListParams`] based on a product name and optional instance
    /// name.
    pub async fn list_services(
        &self,
        namespace: Option<&str>,
        list_params: &ListParams,
    ) -> ListResult<Service> {
        let service_api: Api<Service> = match namespace {
            Some(namespace) => Api::namespaced(self.client.clone(), namespace),
            None => Api::all(self.client.clone()),
        };

        let services = service_api
            .list(list_params)
            .await
            .context(KubeClientFetchSnafu)?;

        Ok(services)
    }

    /// Lists [`Listener`]s by matching labels. The Listeners can be matched by
    /// the product labels. [`ListParamsExt`] provides a utility function to
    /// create [`ListParams`] based on a product name and optional instance
    /// name.
    pub async fn list_listeners(
        &self,
        namespace: Option<&str>,
        list_params: &ListParams,
    ) -> ListResult<Listener> {
        let listener_api: Api<Listener> = match namespace {
            Some(namespace) => Api::namespaced(self.client.clone(), namespace),
            None => Api::all(self.client.clone()),
        };

        let listeners = listener_api
            .list(list_params)
            .await
            .context(KubeClientFetchSnafu)?;

        Ok(listeners)
    }

    /// Retrieves user credentials consisting of username and password from a
    /// secret identified by `secret_name` inside the `secret_namespace`. If
    /// either one of the values is missing, [`Ok(None)`] is returned. An error
    /// is returned if the client failed to get the secret.
    pub async fn get_credentials_from_secret(
        &self,
        secret_name: &str,
        secret_namespace: &str,
        username_key: &str,
        password_key: &str,
    ) -> Result<Credentials> {
        let secret_api: Api<Secret> = Api::namespaced(self.client.clone(), secret_namespace);

        let secret = secret_api
            .get(secret_name)
            .await
            .context(KubeClientFetchSnafu)?;
        let secret_name = secret.name_any();

        let secret_data = secret.data.context(InvalidSecretDataSnafu {
            secret_name: secret_name.clone(),
        })?;

        let username = secret_data
            .get(username_key)
            .context(NoUsernameKeySnafu {
                secret_name: secret_name.clone(),
            })?
            .try_to_string()
            .context(ByteStringConvertSnafu)?;

        let password = secret_data
            .get(password_key)
            .context(NoPasswordKeySnafu { secret_name })?
            .try_to_string()
            .context(ByteStringConvertSnafu)?;

        Ok(Credentials { username, password })
    }

    /// Lists [`Deployment`]s by matching labels. The services can be matched
    /// by the app labels. [`ListParamsExt`] provides a utility function to
    /// create [`ListParams`] based on a app name and other labels.
    pub async fn list_deployments(
        &self,
        namespace: Option<&str>,
        list_params: &ListParams,
    ) -> ListResult<Deployment> {
        let deployment_api: Api<Deployment> = match namespace {
            Some(namespace) => Api::namespaced(self.client.clone(), namespace),
            None => Api::all(self.client.clone()),
        };

        let deployments = deployment_api
            .list(list_params)
            .await
            .context(KubeClientFetchSnafu)?;

        Ok(deployments)
    }

    /// Lists [`StatefulSet`]s by matching labels. The services can be matched
    /// by the app labels. [`ListParamsExt`] provides a utility function to
    /// create [`ListParams`] based on a app name and other labels.
    pub async fn list_stateful_sets(
        &self,
        namespace: Option<&str>,
        list_params: &ListParams,
    ) -> ListResult<StatefulSet> {
        let stateful_set_api: Api<StatefulSet> = match namespace {
            Some(namespace) => Api::namespaced(self.client.clone(), namespace),
            None => Api::all(self.client.clone()),
        };

        let stateful_sets = stateful_set_api
            .list(list_params)
            .await
            .context(KubeClientFetchSnafu)?;

        Ok(stateful_sets)
    }

    pub async fn list_nodes(&self) -> ListResult<Node> {
        let node_api: Api<Node> = Api::all(self.client.clone());

        let nodes = node_api
            .list(&ListParams::default())
            .await
            .context(KubeClientFetchSnafu)?;

        Ok(nodes)
    }

    /// Returns a [`Namespace`] identified by name. If this namespace doesn't
    /// exist, this method returns [`None`].
    pub async fn get_namespace(&self, name: &str) -> Result<Option<Namespace>> {
        let namespace_api: Api<Namespace> = Api::all(self.client.clone());
        namespace_api
            .get_opt(name)
            .await
            .context(KubeClientFetchSnafu)
    }

    /// Creates a [`Namespace`] with `name` in the cluster. This method will
    /// return an error if the namespace already exists. Instead of using this
    /// method directly, it is advised to use [`namespace::create_if_needed`][1]
    /// instead.
    ///
    /// [1]: crate::platform::namespace
    pub async fn create_namespace(&self, name: String) -> Result<()> {
        let namespace_api: Api<Namespace> = Api::all(self.client.clone());
        namespace_api
            .create(&PostParams::default(), &Namespace {
                metadata: ObjectMeta {
                    name: Some(name),
                    ..Default::default()
                },
                ..Default::default()
            })
            .await
            .context(KubeClientPatchSnafu)?;

        Ok(())
    }

    /// Creates a [`Namespace`] only if not already present in the current cluster.
    pub async fn create_namespace_if_needed(&self, name: String) -> Result<()> {
        if self.get_namespace(&name).await?.is_none() {
            self.create_namespace(name).await?
        }

        Ok(())
    }

    /// Retrieves [`ClusterInfo`][cluster::ClusterInfo] which contains resource
    /// information for the current cluster. It should be noted that
    /// [`ClusterInfo`][cluster::ClusterInfo] contains data about allocatable
    /// resources. These values don't reflect currently available resources.
    pub async fn get_cluster_info(&self) -> Result<cluster::ClusterInfo> {
        let nodes = self.list_nodes().await?;
        cluster::ClusterInfo::from_nodes(nodes).context(ClusterInformationSnafu)
    }

    pub async fn get_endpoints(&self, namespace: &str, name: &str) -> Result<Endpoints> {
        let endpoints_api: Api<Endpoints> = Api::namespaced(self.client.clone(), namespace);
        endpoints_api.get(name).await.context(KubeClientFetchSnafu)
    }

    /// Try to resolve the given [`GroupVersionKind`]. In case the resolution fails a discovery is run to pull in new
    /// GVKs that are not present in the [`Discovery`] cache. Afterwards a normal resolution is issued.
    async fn resolve_gvk(
        &self,
        gvk: &GroupVersionKind,
    ) -> Result<Option<(ApiResource, ApiCapabilities)>> {
        let resolved = self.discovery.read().await.resolve_gvk(gvk);

        Ok(match resolved {
            Some(resolved) => Some(resolved),
            None => {
                info!(?gvk, "discovery did not include gvk");

                // We take the lock early here to avoid running multiple discoveries in parallel (as they are expensive)
                let mut old_discovery = self.discovery.write().await;

                // We create a new Discovery object here, as [`Discovery::run`] consumes self
                let new_discovery = Self::run_discovery(self.client.clone()).await?;
                *old_discovery = new_discovery;

                // Release the lock as quickly as possible
                drop(old_discovery);
                self.discovery.read().await.resolve_gvk(gvk)
            }
        })
    }

    /// Creates a new [`Discovery`] object and immediatly runs a discovery.
    #[tracing::instrument(skip_all)]
    async fn run_discovery(client: kube::client::Client) -> Result<Discovery> {
        info!("running discovery");
        Discovery::new(client)
            .run()
            .await
            .context(GVKDiscoveryRunSnafu)
    }

    /// Extracts the [`GroupVersionKind`] from [`TypeMeta`].
    fn gvk_of_typemeta(type_meta: &TypeMeta) -> GroupVersionKind {
        match type_meta.api_version.split_once('/') {
            Some((group, version)) => GroupVersionKind::gvk(group, version, &type_meta.kind),
            None => GroupVersionKind::gvk("", &type_meta.api_version, &type_meta.kind),
        }
    }
}
