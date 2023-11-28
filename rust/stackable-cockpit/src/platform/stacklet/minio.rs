use kube::{api::ListParams, ResourceExt};
use snafu::ResultExt;

use crate::{
    platform::{
        service::get_endpoint_urls,
        stacklet::{Error, KubeClientFetchSnafu, ServiceFetchSnafu, Stacklet},
    },
    utils::k8s::Client,
};

pub(super) async fn list(
    kube_client: &Client,
    namespace: Option<&str>,
) -> Result<Vec<Stacklet>, Error> {
    let mut stacklets = Vec::new();

    // The helm-chart uses `app` instead of `app.kubernetes.io/app`, so we can't use `ListParams::from_product` here
    let params = ListParams::default().labels("app=minio,app.kubernetes.io/managed-by=Helm");
    let services = kube_client
        .list_services(namespace, &params)
        .await
        .context(KubeClientFetchSnafu)?;

    let console_services = services
        .iter()
        .filter(|s| s.name_unchecked().ends_with("-console"));

    for service in console_services {
        let service_name = service.name_any();
        let endpoints = get_endpoint_urls(kube_client, service, &service_name)
            .await
            .context(ServiceFetchSnafu)?;

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
