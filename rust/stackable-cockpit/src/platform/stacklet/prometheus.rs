use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
use kube::{api::ListParams, ResourceExt};
use snafu::ResultExt;

use crate::{
    platform::{
        service::get_service_endpoint_urls,
        stacklet::{KubeSnafu, Stacklet, StackletError},
    },
    utils::k8s::{ConditionsExt, KubeClient},
};

pub(super) async fn list(
    kube_client: &KubeClient,
    namespace: Option<&str>,
) -> Result<Vec<Stacklet>, StackletError> {
    let mut stacklets = Vec::new();

    // The helm-chart uses `app` instead of `app.kubernetes.io/app`, so we can't use `ListParams::from_product` here
    let params = ListParams {
        label_selector: Some("app=kube-prometheus-stack-prometheus".to_string()),
        ..Default::default()
    };

    let services = kube_client
        .list_services(namespace, &params)
        .await
        .context(KubeSnafu)?;

    for service in services {
        let service_name = service.name_any();
        let conditions: Vec<Condition> = match &service.status {
            Some(status) => status.conditions.clone().unwrap_or(vec![]),
            None => vec![],
        };

        let endpoints = get_service_endpoint_urls(kube_client, &service, &service_name)
            .await
            .map_err(|err| StackletError::ServiceError { source: err })?;

        stacklets.push(Stacklet {
            product: "prometheus".to_string(),
            namespace: service.namespace(),
            conditions: conditions.plain(),
            name: service.name_any(),
            endpoints,
        })
    }

    Ok(stacklets)
}
