use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
use kube::{api::ListParams, ResourceExt};
use snafu::ResultExt;

use crate::{
    kube::{ConditionsExt, KubeClient, ListParamsExt, ProductLabel},
    platform::stacklet::{KubeSnafu, Product, StackletError},
};

pub(super) async fn list_products(
    kube_client: &KubeClient,
    namespace: Option<&str>,
) -> Result<Vec<Product>, StackletError> {
    let mut products = Vec::new();

    let params = ListParams::from_product("opensearch-dashboards", None, ProductLabel::Name);
    let services = kube_client
        .list_services(namespace, &params)
        .await
        .context(KubeSnafu {})?;

    for service in services {
        let conditions: Vec<Condition> = match &service.status {
            Some(status) => status.conditions.clone().unwrap_or(vec![]),
            None => vec![],
        };

        products.push(Product {
            name: service.name_any(),
            namespace: service.namespace(),
            conditions: conditions.plain(),
        })
    }

    Ok(products)
}
