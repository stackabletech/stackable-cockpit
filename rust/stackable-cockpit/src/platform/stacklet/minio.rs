use kube::{api::ListParams, ResourceExt};

use crate::{
    platform::{
        service::get_service_endpoint_urls,
        stacklet::{Stacklet, StackletError},
    },
    utils::k8s::KubeClient,
};

pub(super) async fn list(
    kube_client: &KubeClient,
    namespace: Option<&str>,
) -> Result<Vec<Stacklet>, StackletError> {
    let mut stacklets = Vec::new();

    // The helm-chart uses `app` instead of `app.kubernetes.io/app`, so we can't use `ListParams::from_product` here
    let params = ListParams::default().labels("app=minio,app.kubernetes.io/managed-by=Helm");
    let services = kube_client.list_services(namespace, &params).await?;
    let console_services = services
        .iter()
        .filter(|s| s.name_unchecked().ends_with("-console"));

    for service in console_services {
        let service_name = service.name_any();
        let endpoints = get_service_endpoint_urls(kube_client, service, &service_name)
            .await
            .map_err(|err| StackletError::ServiceError { source: err })?;

        stacklets.push(Stacklet {
            product: "minio".to_string(),
            namespace: service.namespace(),
            conditions: Vec::new(),
            name: service.name_any(),
            endpoints,
        })
    }

    Ok(stacklets)
}
