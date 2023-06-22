use indexmap::IndexMap;
use kube::{core::GroupVersionKind, ResourceExt};
use serde::Serialize;
use snafu::{ResultExt, Snafu};
use stackable_operator::status::condition::ClusterCondition;
use tracing::warn;

use crate::{
    constants::PRODUCT_NAMES,
    kube::{ConditionsExt, DisplayCondition, KubeClient, KubeClientError},
    utils::string::Casing,
};

mod grafana;
mod minio;
mod opensearch;
mod prometheus;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    /// Name of the product.
    pub name: String,

    /// Some CRDs are cluster scoped.
    pub namespace: Option<String>,

    /// Multiple cluster conditions
    pub conditions: Vec<DisplayCondition>,
}

#[derive(Debug, Snafu)]
pub enum StackletError {
    #[snafu(display("kubernetes error"))]
    KubeError { source: KubeClientError },

    #[snafu(display("no namespace set for custom resource '{crd_name}'"))]
    CustomCrdNamespaceError { crd_name: String },

    #[snafu(display("JSON error"))]
    JsonError { source: serde_json::Error },
}

pub type StackletList = IndexMap<String, Vec<Product>>;

/// Lists all installed stacklets. If `namespace` is [`None`], stacklets from ALL
/// namespaces are returned. If `namespace` is [`Some`], only stacklets installed
/// in the specified namespace are returned. The `options` allow further
/// customization of the returned information.
pub async fn list_stacklets(namespace: Option<&str>) -> Result<StackletList, StackletError> {
    let kube_client = KubeClient::new().await.context(KubeSnafu {})?;

    let mut stacklets = list_stackable_stacklets(&kube_client, namespace).await?;

    let grafana_products = grafana::list_products(&kube_client, namespace).await?;
    if !grafana_products.is_empty() {
        stacklets.insert("grafana".into(), grafana_products);
    }

    let minio_products = minio::list_products(&kube_client, namespace).await?;
    if !minio_products.is_empty() {
        stacklets.insert("minio".into(), minio_products);
    }

    let opensearch_products = opensearch::list_products(&kube_client, namespace).await?;
    if !opensearch_products.is_empty() {
        stacklets.insert("opensearch-dashboards".into(), opensearch_products);
    }

    let prometheus_products = prometheus::list_products(&kube_client, namespace).await?;
    if !prometheus_products.is_empty() {
        stacklets.insert("prometheus".into(), prometheus_products);
    }

    Ok(stacklets)
}

async fn list_stackable_stacklets(
    kube_client: &KubeClient,
    namespace: Option<&str>,
) -> Result<StackletList, StackletError> {
    let product_list = build_products_gvk_list(PRODUCT_NAMES);
    let mut stacklets = StackletList::new();

    for (product_name, product_gvk) in product_list {
        let objects = match kube_client
            .list_objects(&product_gvk, namespace)
            .await
            .context(KubeSnafu {})?
        {
            Some(obj) => obj,
            None => {
                warn!(
                    "Failed to list services because the gvk {product_gvk:?} can not be resolved"
                );
                continue;
            }
        };

        let mut products = Vec::new();

        for object in objects {
            let conditions: Vec<ClusterCondition> = match object.data.pointer("/status/conditions")
            {
                Some(conditions) => {
                    serde_json::from_value(conditions.clone()).context(JsonSnafu {})?
                }
                None => vec![],
            };

            let object_name = object.name_any();
            let object_namespace = match object.namespace() {
                Some(ns) => ns,
                // If the custom resource does not have a namespace set it can't expose a service
                None => continue,
            };

            products.push(Product {
                namespace: Some(object_namespace),
                name: object_name,
                conditions: conditions.plain(),
            });
        }

        if !products.is_empty() {
            stacklets.insert(product_name.to_string(), products);
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

        map.insert(
            *product_name,
            GroupVersionKind {
                group: format!("{product_name}.stackable.tech"),
                version: "v1alpha1".into(),
                kind: format!("{}Cluster", product_name.capitalize()),
            },
        );
    }

    map
}
