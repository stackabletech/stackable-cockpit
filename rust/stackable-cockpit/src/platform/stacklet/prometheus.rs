use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
use kube::{api::ListParams, ResourceExt};
use snafu::ResultExt;

use crate::{
    platform::stacklet::{KubeSnafu, Stacklet, StackletError},
    utils::k8s::{ConditionsExt, KubeClient, ListParamsExt, ProductLabel},
};

pub(super) async fn list(
    kube_client: &KubeClient,
    namespace: Option<&str>,
) -> Result<Vec<Stacklet>, StackletError> {
    let mut stacklets = Vec::new();

    let params = ListParams::from_product(
        "kube-prometheus-stack-prometheus",
        Some("prometheus"),
        ProductLabel::App,
    );

    let services = kube_client
        .list_services(namespace, &params)
        .await
        .context(KubeSnafu)?;

    for service in services {
        let conditions: Vec<Condition> = match &service.status {
            Some(status) => status.conditions.clone().unwrap_or(vec![]),
            None => vec![],
        };

        stacklets.push(Stacklet {
            name: service.name_any(),
            namespace: service.namespace(),
            product: "prometheus".to_string(),
            conditions: conditions.plain(),
        })
    }

    Ok(stacklets)
}
