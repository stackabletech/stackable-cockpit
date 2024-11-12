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
    platform::{credentials, service},
    utils::{
        k8s::{self, Client, ConditionsExt},
        string::Casing,
    },
};

mod grafana;
mod minio;
mod opensearch;
mod prometheus;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
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
    pub conditions: Vec<ClusterCondition>,

    /// Multiple cluster conditions meant for displaying in CLI.
    pub display_conditions: Vec<k8s::DisplayCondition>,
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("failed to create Kubernetes client"))]
    KubeClientCreate { source: k8s::Error },

    #[snafu(display("failed to fetch data from the Kubernetes API"))]
    KubeClientFetch { source: k8s::Error },

    #[snafu(display("no namespace set for custom resource '{crd_name}'"))]
    CustomCrdNamespace { crd_name: String },

    #[snafu(display("failed to deserialize cluster conditions from JSON"))]
    DeserializeConditions { source: serde_json::Error },

    #[snafu(display("failed to receive service information"))]
    ServiceFetch { source: service::Error },
}

/// Lists all installed stacklets. If `namespace` is [`None`], stacklets from ALL
/// namespaces are returned. If `namespace` is [`Some`], only stacklets installed
/// in the specified namespace are returned. The `options` allow further
/// customization of the returned information.
pub async fn list_stacklets(
    client: &Client,
    namespace: Option<&str>,
) -> Result<Vec<Stacklet>, Error> {
    let mut stacklets = list_stackable_stacklets(client, namespace).await?;
    stacklets.extend(grafana::list(client, namespace).await?);
    stacklets.extend(minio::list(client, namespace).await?);
    stacklets.extend(opensearch::list(client, namespace).await?);
    stacklets.extend(prometheus::list(client, namespace).await?);

    Ok(stacklets)
}

pub async fn get_credentials_for_product(
    client: &Client,
    namespace: &str,
    object_name: &str,
    product_name: &str,
) -> Result<Option<credentials::Credentials>, Error> {
    let product_gvk = gvk_from_product_name(product_name);
    let product_cluster = match client
        .get_namespaced_object(namespace, object_name, &product_gvk)
        .await
        .context(KubeClientFetchSnafu)?
    {
        Some(obj) => obj,
        None => {
            info!(
                "Failed to retrieve credentials because the gvk {product_gvk:?} cannot be resolved"
            );
            return Ok(None);
        }
    };

    let credentials = match credentials::get(client, product_name, &product_cluster).await {
        Ok(credentials) => credentials,
        Err(credentials::Error::NoSecret) => None,
        Err(credentials::Error::KubeClientFetch { source }) => {
            return Err(Error::KubeClientFetch { source })
        }
    };

    Ok(credentials)
}

async fn list_stackable_stacklets(
    client: &Client,
    namespace: Option<&str>,
) -> Result<Vec<Stacklet>, Error> {
    let product_list = build_products_gvk_list(PRODUCT_NAMES);
    let mut stacklets = Vec::new();

    for (product_name, product_gvk) in product_list {
        let objects = match client
            .list_objects(&product_gvk, namespace)
            .await
            .context(KubeClientFetchSnafu)?
        {
            Some(obj) => obj,
            None => {
                info!(
                    "Failed to list stacklets because the gvk {product_gvk:?} can not be resolved"
                );
                continue;
            }
        };

        for object in objects {
            let conditions: Vec<ClusterCondition> = match object.data.pointer("/status/conditions")
            {
                Some(conditions) => serde_json::from_value(conditions.clone())
                    .context(DeserializeConditionsSnafu)?,
                None => vec![],
            };

            let object_name = object.name_any();
            let object_namespace = match object.namespace() {
                Some(ns) => ns,
                // If the custom resource does not have a namespace set it can't expose a service
                None => continue,
            };

            let endpoints =
                service::get_endpoints(client, product_name, &object_name, &object_namespace)
                    .await
                    .context(ServiceFetchSnafu)?;

            stacklets.push(Stacklet {
                namespace: Some(object_namespace),
                product: product_name.to_string(),
                name: object_name,
                endpoints,
                conditions: conditions.clone(),
                display_conditions: conditions.plain(),
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

// FIXME: Support SparkApplication
fn gvk_from_product_name(product_name: &str) -> GroupVersionKind {
    GroupVersionKind {
        group: format!("{product_name}.stackable.tech"),
        version: "v1alpha1".into(),
        kind: format!("{}Cluster", product_name.capitalize()),
    }
}
