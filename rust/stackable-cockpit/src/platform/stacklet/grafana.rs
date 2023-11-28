use kube::{api::ListParams, ResourceExt};
use snafu::ResultExt;

use crate::{
    platform::{
        service::get_endpoint_urls,
        stacklet::{Error, KubeClientFetchSnafu, ServiceFetchSnafu, Stacklet},
    },
    utils::k8s::{Client, ListParamsExt, ProductLabel},
};

pub(super) async fn list(
    kube_client: &Client,
    namespace: Option<&str>,
) -> Result<Vec<Stacklet>, Error> {
    let mut stacklets = Vec::new();

    let params = ListParams::from_product("grafana", None, ProductLabel::Name);
    let services = kube_client
        .list_services(namespace, &params)
        .await
        .context(KubeClientFetchSnafu)?;

    for service in services {
        let service_name = service.name_any();
        let endpoints = get_endpoint_urls(kube_client, &service, &service_name)
            .await
            .context(ServiceFetchSnafu)?;

        stacklets.push(Stacklet {
            conditions: Vec::new(),
            namespace: service.namespace(),
            product: "grafana".to_string(),
            name: service_name,
            endpoints,
        })
    }

    Ok(stacklets)
}
