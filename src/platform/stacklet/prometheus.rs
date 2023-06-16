use kube::ResourceExt;
use snafu::ResultExt;

use crate::{
    kube::{KubeClient, ProductLabel},
    platform::stacklet::{KubeSnafu, Product, StackletError},
};

pub(super) async fn list_products(
    kube_client: &KubeClient,
    namespace: Option<&str>,
) -> Result<Vec<Product>, StackletError> {
    let mut products = Vec::new();

    let services = kube_client
        .list_services(
            namespace,
            "kube-prometheus-stack-prometheus",
            Some("prometheus"),
            ProductLabel::App,
        )
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
