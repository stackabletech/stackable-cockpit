use std::string::FromUtf8Error;

use indexmap::IndexMap;
use k8s_openapi::api::core::v1::{Endpoints, Secret, Service};
use kube::{
    api::{ListParams, Patch, PatchParams},
    core::{DynamicObject, GroupVersionKind, ObjectList, TypeMeta},
    discovery::Scope,
    Api, Client, Discovery, ResourceExt,
};
use serde::Deserialize;
use snafu::{ResultExt, Snafu};

use crate::constants::REDACTED_PASSWORD;

#[derive(Debug, Snafu)]
pub enum KubeError {
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

pub struct KubeClient {
    client: Client,
    discovery: Discovery,
}

impl KubeClient {
    pub async fn new() -> Result<Self, KubeError> {
        let client = Client::try_default().await.context(KubeSnafu {})?;
        let discovery = Discovery::new(client.clone())
            .run()
            .await
            .context(KubeSnafu {})?;

        Ok(Self { client, discovery })
    }

    pub async fn deploy_manifests(
        &self,
        manifests: &str,
        namespace: &str,
    ) -> Result<(), KubeError> {
        for manifest in serde_yaml::Deserializer::from_str(manifests) {
            let mut object = DynamicObject::deserialize(manifest).context(YamlSnafu {})?;
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
            .context(KubeSnafu {})?;
        }

        Ok(())
    }

    pub async fn list_objects(
        &self,
        gvk: &GroupVersionKind,
        namespace: Option<&str>,
    ) -> Result<Option<ObjectList<DynamicObject>>, KubeError> {
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
            .context(KubeSnafu {})?;

        Ok(Some(objects))
    }

    pub async fn list_services(
        &self,
        namespace: &str,
        product_name: &str,
        object_name: &str,
    ) -> Result<ObjectList<Service>, KubeError> {
        let service_api: Api<Service> = Api::namespaced(self.client.clone(), namespace);

        let service_list_params = ListParams::default()
            .labels(format!("app.kubernetes.io/name={product_name}").as_str())
            .labels(format!("app.kubernetes.io/instance={object_name}").as_str());

        let services = service_api
            .list(&service_list_params)
            .await
            .context(KubeSnafu {})?;

        Ok(services)
    }

    pub async fn list_service_endpoints(
        &self,
        service: &Service,
        _object_name: &str,
    ) -> Result<IndexMap<String, String>, KubeError> {
        let namespace = service.namespace().ok_or(
            MissingServiceNamespaceSnafu {
                service: service.name_any(),
            }
            .build(),
        )?;

        // TODO (Techassi): Get rid of this potential panic
        let service_name = service.name_unchecked();

        let endpoints_api: Api<Endpoints> = Api::namespaced(self.client.clone(), &namespace);
        let _endpoints = endpoints_api
            .get(&service_name)
            .await
            .context(KubeSnafu {})?;

        todo!()
    }

    pub async fn get_credentials_from_secret(
        &self,
        secret_name: &str,
        secret_namespace: &str,
        username_key: &str,
        password_key: Option<&str>,
    ) -> Result<Option<(String, String)>, KubeError> {
        let secret_api: Api<Secret> = Api::namespaced(self.client.clone(), secret_namespace);

        let secret = secret_api.get(secret_name).await.context(KubeSnafu {})?;
        let secret_data = secret.data.ok_or(InvalidSecretDataSnafu {}.build())?;

        let username = match secret_data.get(username_key) {
            Some(username) => {
                String::from_utf8(username.0.clone()).context(ByteStringConvertSnafu {})?
            }
            None => return Ok(None),
        };

        let password = match password_key {
            Some(key) => match secret_data.get(key) {
                Some(password) => {
                    String::from_utf8(password.0.clone()).context(ByteStringConvertSnafu {})?
                }
                None => return Ok(None),
            },
            None => REDACTED_PASSWORD.to_string(),
        };

        Ok(Some((username, password)))
    }

    fn gvk_of_typemeta(type_meta: &TypeMeta) -> GroupVersionKind {
        match type_meta.api_version.split_once('/') {
            Some((group, version)) => GroupVersionKind::gvk(group, version, &type_meta.kind),
            None => GroupVersionKind::gvk("", &type_meta.api_version, &type_meta.kind),
        }
    }
}
