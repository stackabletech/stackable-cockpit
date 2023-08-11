use indexmap::IndexMap;
use kube::{api::ListParams, ResourceExt};
use snafu::ResultExt;

use crate::{
    platform::stacklet::{KubeSnafu, Stacklet, StackletError},
    utils::k8s::{KubeClient, ListParamsExt, ProductLabel},
};

pub(super) async fn list(
    kube_client: &KubeClient,
    namespace: Option<&str>,
) -> Result<Vec<Stacklet>, StackletError> {
    let mut stacklets = Vec::new();

    let params = ListParams::from_product("opensearch-dashboards", None, ProductLabel::Name);
    let services = kube_client
        .list_services(namespace, &params)
        .await
        .context(KubeSnafu)?;

    for service in services {
        stacklets.push(Stacklet {
            name: service.name_any(),
            namespace: service.namespace(),
            product: "opensearch-dashboards".to_string(),
            endpoints: IndexMap::new(),
            conditions: Vec::new(),
        })
    }

    Ok(stacklets)
}
