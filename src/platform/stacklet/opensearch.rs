use kube::{api::ListParams, ResourceExt};
use snafu::ResultExt;

use crate::{
    kube::{KubeClient, ListParamsExt, ProductLabel},
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
        products.push(Product {
            name: service.name_any(),
            namespace: service.namespace(),
            conditions: vec![],
        })
    }

    Ok(products)
}
