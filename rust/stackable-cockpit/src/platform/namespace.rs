use crate::kube::{KubeClient, KubeClientError};

/// Creates a namespace with `name` if needed (not already present in the
/// cluster).
pub async fn create_if_needed(name: String) -> Result<(), KubeClientError> {
    let client = KubeClient::new().await?;
    let namespaces = client.list_namespaces().await?;

    let exists = namespaces.iter().any(|ns| match &ns.metadata.name {
        Some(ns_name) => ns_name == &name,
        None => false,
    });

    if !exists {
        client.create_namespace(name).await?
    }

    Ok(())
}
