use indexmap::IndexMap;
use kube::{core::GroupVersionKind, ResourceExt};
use serde::Serialize;
use snafu::{ResultExt, Snafu};
use stackable_operator::status::condition::ClusterCondition;
use tracing::info;

#[cfg(feature = "openapi")]
use utoipa::ToSchema;

use crate::{
    constants::PRODUCT_NAMES,
    platform::{
        credentials::{get_credentials, Credentials, CredentialsError},
        service::{get_service_endpoints, ServiceError},
    },
    utils::{
        k8s::{ConditionsExt, DisplayCondition, KubeClient, KubeClientError},
        string::Casing,
    },
};

mod grafana;
mod minio;
mod opensearch;
mod prometheus;

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct Stacklet {
    /// Name of the stacklet.
    pub name: String,

    /// Some CRDs are cluster scoped.
    pub namespace: Option<String>,

    /// Name of the product.
    pub product: String,

    /// Endpoint addresses the product is reachable at.
    /// The key is the service name (e.g. `web-ui`), the value is the URL.
    pub endpoints: IndexMap<String, String>,

    /// Multiple cluster conditions.
    pub conditions: Vec<DisplayCondition>,
}

#[derive(Debug, Snafu)]
pub enum StackletError {
    #[snafu(display("kubernetes error"), context(false))]
    KubeError { source: KubeClientError },

    #[snafu(display("no namespace set for custom resource '{crd_name}'"))]
    CustomCrdNamespaceError { crd_name: String },

    #[snafu(display("JSON error"))]
    JsonError { source: serde_json::Error },

    #[snafu(display("service error"))]
    ServiceError { source: ServiceError },
}

/// Lists all installed stacklets. If `namespace` is [`None`], stacklets from ALL
/// namespaces are returned. If `namespace` is [`Some`], only stacklets installed
/// in the specified namespace are returned. The `options` allow further
/// customization of the returned information.
pub async fn list_stacklets(namespace: Option<&str>) -> Result<Vec<Stacklet>, StackletError> {
    let kube_client = KubeClient::new().await?;

    let mut stacklets = list_stackable_stacklets(&kube_client, namespace).await?;
    stacklets.extend(grafana::list(&kube_client, namespace).await?);
    stacklets.extend(minio::list(&kube_client, namespace).await?);
    stacklets.extend(opensearch::list(&kube_client, namespace).await?);
    stacklets.extend(prometheus::list(&kube_client, namespace).await?);

    Ok(stacklets)
}

pub async fn get_credentials_for_product(
    object_name: &str,
    product_name: &str,
    namespace: Option<&str>,
) -> Result<Option<Credentials>, StackletError> {
    let kube_client = KubeClient::new().await?;

    let product_gvk = gvk_from_product_name(product_name);
    let object = match kube_client
        .get_object(object_name, &product_gvk, namespace)
        .await?
    {
        Some(obj) => obj,
        None => {
            info!(
                "Failed to retrieve credentials because the gvk {product_gvk:?} cannot be resolved"
            );
            return Ok(None);
        }
    };

    let credentials = match get_credentials(&kube_client, product_name, &object).await {
        Ok(credentials) => credentials,
        Err(err) => match err {
            CredentialsError::KubeError { source } => return Err(source.into()),
            CredentialsError::NoSecret => None,
        },
    };

    Ok(credentials)
}

async fn list_stackable_stacklets(
    kube_client: &KubeClient,
    namespace: Option<&str>,
) -> Result<Vec<Stacklet>, StackletError> {
    let product_list = build_products_gvk_list(PRODUCT_NAMES);
    let mut stacklets = Vec::new();

    for (product_name, product_gvk) in product_list {
        let objects = match kube_client.list_objects(&product_gvk, namespace).await? {
            Some(obj) => obj,
            None => {
                info!(
                    "Failed to list services because the gvk {product_gvk:?} can not be resolved"
                );
                continue;
            }
        };

        for object in objects {
            let conditions: Vec<ClusterCondition> = match object.data.pointer("/status/conditions")
            {
                Some(conditions) => {
                    serde_json::from_value(conditions.clone()).context(JsonSnafu)?
                }
                None => vec![],
            };

            let object_name = object.name_any();
            let object_namespace = match object.namespace() {
                Some(ns) => ns,
                // If the custom resource does not have a namespace set it can't expose a service
                None => continue,
            };

            let endpoints =
                get_service_endpoints(kube_client, product_name, &object_name, &object_namespace)
                    .await
                    .context(ServiceSnafu)?;

            stacklets.push(Stacklet {
                namespace: Some(object_namespace),
                product: product_name.to_string(),
                conditions: conditions.plain(),
                name: object_name,
                endpoints,
            });
        }
    }

    Ok(stacklets)
}

fn build_products_gvk_list<'a>(product_names: &[&'a str]) -> IndexMap<&'a str, GroupVersionKind> {
    let mut map = IndexMap::new();

    for product_name in product_names {
        // Why? Just why? Can we please make this consistent?
        if *product_name == "spark-history-server" {
            map.insert(
                *product_name,
                GroupVersionKind {
                    group: "spark.stackable.tech".into(),
                    version: "v1alpha1".into(),
                    kind: "SparkHistoryServer".into(),
                },
            );
            continue;
        }

        map.insert(*product_name, gvk_from_product_name(product_name));
    }

    map
}

fn gvk_from_product_name(product_name: &str) -> GroupVersionKind {
    GroupVersionKind {
        group: format!("{product_name}.stackable.tech"),
        version: "v1alpha1".into(),
        kind: format!("{}Cluster", product_name.capitalize()),
    }
}
