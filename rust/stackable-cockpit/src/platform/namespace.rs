use crate::kube::{KubeClient, KubeClientError};

/// Creates a namespace with `name` if needed (not already present in the
/// cluster).
pub async fn create_if_needed(name: String) -> Result<(), KubeClientError> {
    let client = KubeClient::new().await?;
    client.create_namespace_if_needed(name).await
}
