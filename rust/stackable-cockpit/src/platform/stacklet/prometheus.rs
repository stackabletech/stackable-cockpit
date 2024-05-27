use kube::{api::ListParams, ResourceExt};
use snafu::ResultExt;

use crate::{
    platform::{
        service::get_endpoint_urls,
        stacklet::{Error, KubeClientFetchSnafu, ServiceFetchSnafu, Stacklet},
    },
    utils::k8s::Client,
};

pub(super) async fn list(client: &Client, namespace: Option<&str>) -> Result<Vec<Stacklet>, Error> {
    let mut stacklets = Vec::new();

    // The helm-chart uses `app` instead of `app.kubernetes.io/app`, so we can't use `ListParams::from_product` here
    let params = ListParams::default().labels("app=kube-prometheus-stack-prometheus");
    let services = client
        .list_services(namespace, &params)
        .await
        .context(KubeClientFetchSnafu)?;

    for service in services {
        let service_name = service.name_any();
        let endpoints = get_endpoint_urls(client, &service, &service_name)
            .await
            .context(ServiceFetchSnafu)?;

        stacklets.push(Stacklet {
            product: "prometheus".to_string(),
            namespace: service.namespace(),
            conditions: Vec::new(),
            name: service.name_any(),
            endpoints,
        })
    }

    Ok(stacklets)
}
