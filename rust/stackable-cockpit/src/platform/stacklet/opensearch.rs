use kube::{ResourceExt, api::ListParams};
use snafu::ResultExt;

use crate::{
    platform::{
        service::get_endpoint_urls,
        stacklet::{Error, KubeClientFetchSnafu, ServiceFetchSnafu, Stacklet},
    },
    utils::k8s::{Client, ListParamsExt, ProductLabel},
};

pub(super) async fn list(client: &Client, namespace: Option<&str>) -> Result<Vec<Stacklet>, Error> {
    let mut stacklets = Vec::new();

    let params = ListParams::from_product("opensearch-dashboards", None, ProductLabel::Name);
    let services = client
        .list_services(namespace, &params)
        .await
        .map_err(Box::new)
        .context(KubeClientFetchSnafu)?;

    for service in services {
        let service_name = service.name_any();
        let endpoints = get_endpoint_urls(client, &service, &service_name)
            .await
            .context(ServiceFetchSnafu)?;

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
