use indexmap::IndexMap;
use k8s_openapi::api::apps::v1::{DeploymentCondition, StatefulSetCondition};
use kube::{api::ListParams, ResourceExt};
use snafu::ResultExt;

use crate::{
    platform::stacklet::{KubeSnafu, Stacklet, StackletError},
    utils::k8s::{ConditionsExt, KubeClient},
};

pub(super) async fn list(
    kube_client: &KubeClient,
    namespace: Option<&str>,
) -> Result<Vec<Stacklet>, StackletError> {
    let params = ListParams::default().labels("app=minio");
    let mut stacklets = Vec::new();

    // MinIO can either be installed in standalone mode which creates a Deployment
    // The other option is to run it in a distributed mode, which created a StatefulSet
    // So we have to check for both
    let deployments = kube_client
        .list_deployments(namespace, &params)
        .await
        .context(KubeSnafu)?;

    for deployment in deployments {
        let conditions: Vec<DeploymentCondition> = match &deployment.status {
            Some(status) => status.conditions.clone().unwrap_or(vec![]),
            None => vec![],
        };

        stacklets.push(Stacklet {
            name: deployment.name_any(),
            namespace: deployment.namespace(),
            product: "minio".to_string(),
            endpoints: IndexMap::new(),
            conditions: conditions.plain(),
        })
    }

    let stateful_sets = kube_client
        .list_stateful_sets(namespace, &params)
        .await
        .context(KubeSnafu)?;

    for stateful_set in stateful_sets {
        let conditions: Vec<StatefulSetCondition> = match &stateful_set.status {
            Some(status) => status.conditions.clone().unwrap_or(vec![]),
            None => vec![],
        };

        stacklets.push(Stacklet {
            name: stateful_set.name_any(),
            namespace: stateful_set.namespace(),
            product: "minio".to_string(),
            endpoints: IndexMap::new(),
            conditions: conditions.plain(),
        })
    }

    Ok(stacklets)
}
