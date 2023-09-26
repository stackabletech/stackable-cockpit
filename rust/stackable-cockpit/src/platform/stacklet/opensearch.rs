use kube::{api::ListParams, ResourceExt};

use crate::{
    platform::{
        service::get_service_endpoint_urls,
        stacklet::{Stacklet, StackletError},
    },
    utils::k8s::{KubeClient, ListParamsExt, ProductLabel},
};

pub(super) async fn list(
    kube_client: &KubeClient,
    namespace: Option<&str>,
) -> Result<Vec<Stacklet>, StackletError> {
    let mut stacklets = Vec::new();

    let params = ListParams::from_product("opensearch-dashboards", None, ProductLabel::Name);
    let services = kube_client.list_services(namespace, &params).await?;

    for service in services {
        let service_name = service.name_any();
        let endpoints = get_service_endpoint_urls(kube_client, &service, &service_name)
            .await
            .map_err(|err| StackletError::ServiceError { source: err })?;

        // TODO: Add "Logs view" extra info from old stackablectl once "Extra info" field  is supported.
        // see https://github.com/stackabletech/stackablectl/blob/eda45945cfcf5c6581cf1b88c782d98fada8065f/src/services/opensearch.rs#L41

        stacklets.push(Stacklet {
            conditions: Vec::new(),
            namespace: service.namespace(),
            product: "opensearch-dashboards".to_string(),
            name: service_name,
            endpoints,
        });
    }

    Ok(stacklets)
}
