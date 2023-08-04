use std::string::FromUtf8Error;

use k8s_openapi::{
    api::{
        apps::v1::{Deployment, DeploymentCondition, StatefulSet, StatefulSetCondition},
        core::v1::{Namespace, Secret, Service},
    },
    apimachinery::pkg::apis::meta::v1::Condition,
};
use kube::{
    api::{ListParams, Patch, PatchParams},
    core::{DynamicObject, GroupVersionKind, ObjectList, TypeMeta},
    discovery::Scope,
    Api, Client, Discovery, ResourceExt,
};
use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};
use stackable_operator::status::condition::ClusterCondition;

#[cfg(feature = "openapi")]
use utoipa::ToSchema;

use crate::constants::REDACTED_PASSWORD;

pub type ListResult<T, E = KubeClientError> = Result<ObjectList<T>, E>;
pub type Result<T, E = KubeClientError> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum KubeClientError {
    #[snafu(display("kube error: {source}"))]
    KubeError { source: kube::error::Error },

    #[snafu(display("yaml error: {source}"))]
    YamlError { source: serde_yaml::Error },

    #[snafu(display("failed to deploy manifest because type of object {object:?} is not set"))]
    ObjectTypeError { object: DynamicObject },

    #[snafu(display("failed to deploy manifest because GVK {gvk:?} cannot be resolved"))]
    DiscoveryError { gvk: GroupVersionKind },

    #[snafu(display("invalid secret data (empty)"))]
    InvalidSecretData,

    #[snafu(display("failed to convert byte string into UTF-8 string"))]
    ByteStringConvertError { source: FromUtf8Error },

    #[snafu(display("missing namespace for service '{service}'"))]
    MissingServiceNamespace { service: String },
}

pub enum ProductLabel {
    Both,
    Name,
    App,
}

pub struct KubeClient {
    client: Client,
    discovery: Discovery,
}

impl KubeClient {
    /// Tries to create a new default Kubernetes client and immediately runs
    /// a discovery.
    pub async fn new() -> Result<Self> {
        let client = Client::try_default().await.context(KubeSnafu)?;
        let discovery = Discovery::new(client.clone())
            .run()
            .await
            .context(KubeSnafu)?;

        Ok(Self { client, discovery })
    }

    /// Deploys manifests defined the in raw `manifests` YAML string. This
    /// method will fail if it is unable to parse the manifests, unable to
    /// resolve GVKs or unable to patch the dynamic objects.
    pub async fn deploy_manifests(&self, manifests: &str, namespace: &str) -> Result<()> {
        for manifest in serde_yaml::Deserializer::from_str(manifests) {
            let mut object = DynamicObject::deserialize(manifest).context(YamlSnafu)?;
            let object_type = object.types.as_ref().ok_or(
                ObjectTypeSnafu {
                    object: object.clone(),
                }
                .build(),
            )?;

            let gvk = Self::gvk_of_typemeta(object_type);
            let (resource, capabilities) = self
                .discovery
                .resolve_gvk(&gvk)
                .ok_or(DiscoverySnafu { gvk }.build())?;

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
            .context(KubeSnafu)?;
        }

        Ok(())
    }

    /// List objects by looking up a GVK via the discovery. It returns an
    /// optional list of dynamic objects. The method returns [`Ok(None)`]
    /// if the client was unable to resolve the GVK. An error is returned
    /// when the client failed to list the objects.
    pub async fn list_objects(
        &self,
        gvk: &GroupVersionKind,
        namespace: Option<&str>,
    ) -> Result<Option<ObjectList<DynamicObject>>, KubeClientError> {
        let object_api_resource = match self.discovery.resolve_gvk(gvk) {
            Some((object_api_resource, _)) => object_api_resource,
            None => {
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
            .context(KubeSnafu)?;

        Ok(Some(objects))
    }

    /// List services by matching labels. The services can me matched by the
    /// product labels. [`ListParamsExt`] provides a utility function to
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

        let services = service_api.list(list_params).await.context(KubeSnafu)?;
        Ok(services)
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
        password_key: Option<&str>,
    ) -> Result<Option<(String, String)>> {
        let secret_api: Api<Secret> = Api::namespaced(self.client.clone(), secret_namespace);

        let secret = secret_api.get(secret_name).await.context(KubeSnafu)?;
        let secret_data = secret.data.ok_or(InvalidSecretDataSnafu {}.build())?;

        let username = match secret_data.get(username_key) {
            Some(username) => {
                String::from_utf8(username.0.clone()).context(ByteStringConvertSnafu)?
            }
            None => return Ok(None),
        };

        let password = match password_key {
            Some(key) => match secret_data.get(key) {
                Some(password) => {
                    String::from_utf8(password.0.clone()).context(ByteStringConvertSnafu)?
                }
                None => return Ok(None),
            },
            None => REDACTED_PASSWORD.to_string(),
        };

        Ok(Some((username, password)))
    }

    pub async fn list_deployments(
        &self,
        namespace: Option<&str>,
        list_params: &ListParams,
    ) -> ListResult<Deployment> {
        let deployment_api: Api<Deployment> = match namespace {
            Some(namespace) => Api::namespaced(self.client.clone(), namespace),
            None => Api::all(self.client.clone()),
        };

        let deployments = deployment_api.list(list_params).await.context(KubeSnafu)?;

        Ok(deployments)
    }

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
            .context(KubeSnafu)?;

        Ok(stateful_sets)
    }

    pub async fn list_namespaces(&self) -> ListResult<Namespace> {
        let namespace_api: Api<Namespace> = Api::all(self.client.clone());
        let namespaces = namespace_api
            .list(&ListParams::default())
            .await
            .context(KubeSnafu)?;

        Ok(namespaces)
    }

    /// Extracts the GVK from [`TypeMeta`].
    fn gvk_of_typemeta(type_meta: &TypeMeta) -> GroupVersionKind {
        match type_meta.api_version.split_once('/') {
            Some((group, version)) => GroupVersionKind::gvk(group, version, &type_meta.kind),
            None => GroupVersionKind::gvk("", &type_meta.api_version, &type_meta.kind),
        }
    }
}

pub trait ListParamsExt {
    fn from_product(
        product_name: &str,
        instance_name: Option<&str>,
        product_label: ProductLabel,
    ) -> ListParams {
        let mut params = ListParams::default();

        if matches!(product_label, ProductLabel::Name | ProductLabel::Both) {
            params.add_label(format!("app.kubernetes.io/name={product_name}"));
        }

        if matches!(product_label, ProductLabel::App | ProductLabel::Both) {
            params.add_label(format!("app.kubernetes.io/app={product_name}"));
        }

        if let Some(instance_name) = instance_name {
            // NOTE (Techassi): This bothers me a little, but .labels consumes self
            params.add_label(format!("app.kubernetes.io/instance={instance_name}"));
        }

        params
    }

    /// Adds a label to the label selectors.
    fn add_label(&mut self, label: impl Into<String>);
}

impl ListParamsExt for ListParams {
    fn add_label(&mut self, label: impl Into<String>) {
        match self.label_selector.as_mut() {
            Some(labels) => labels.push_str(format!(",{}", label.into()).as_str()),
            None => self.label_selector = Some(label.into()),
        }
    }
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct DisplayCondition {
    pub message: Option<String>,
    pub is_good: Option<bool>,
    pub condition: String,
}

impl DisplayCondition {
    pub fn new(condition: String, message: Option<String>, is_good: Option<bool>) -> Self {
        Self {
            condition,
            message,
            is_good,
        }
    }
}

/// This trait unifies the different conditions, like [`Condition`],
/// [`DeploymentCondition`], [`ClusterCondition`]. The method `plain` returns
/// a plain text representation of the list of conditions. This list ist suited
/// for terminal output, i.e. stackablectl.
pub trait ConditionsExt
where
    Self: IntoIterator,
    Self::Item: ConditionExt,
{
    /// Returns a plain list of conditions.
    fn plain(&self) -> Vec<DisplayCondition>;
}

impl ConditionsExt for Vec<Condition> {
    fn plain(&self) -> Vec<DisplayCondition> {
        self.iter()
            .map(|c| {
                DisplayCondition::new(
                    format!("{}: {}", c.type_, c.status),
                    Some(c.message.clone()),
                    c.is_good(),
                )
            })
            .collect()
    }
}

impl ConditionsExt for Vec<DeploymentCondition> {
    fn plain(&self) -> Vec<DisplayCondition> {
        self.iter()
            .map(|c| {
                DisplayCondition::new(
                    format!("{}: {}", c.type_, c.status),
                    c.message.clone(),
                    c.is_good(),
                )
            })
            .collect()
    }
}

impl ConditionsExt for Vec<ClusterCondition> {
    fn plain(&self) -> Vec<DisplayCondition> {
        self.iter()
            .map(|c| DisplayCondition::new(c.display_short(), c.message.clone(), Some(c.is_good())))
            .collect()
    }
}

impl ConditionsExt for Vec<StatefulSetCondition> {
    fn plain(&self) -> Vec<DisplayCondition> {
        self.iter()
            .map(|c| {
                DisplayCondition::new(
                    format!("{}: {}", c.type_, c.status),
                    c.message.clone(),
                    c.is_good(),
                )
            })
            .collect()
    }
}

pub trait ConditionExt {
    fn is_good(&self) -> Option<bool> {
        None
    }
}

impl ConditionExt for StatefulSetCondition {}
impl ConditionExt for DeploymentCondition {}
impl ConditionExt for Condition {}

impl ConditionExt for ClusterCondition {
    fn is_good(&self) -> Option<bool> {
        Some(self.is_good())
    }
}
