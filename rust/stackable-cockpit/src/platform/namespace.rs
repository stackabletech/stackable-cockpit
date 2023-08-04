use crate::kube::KubeClient;

pub async fn list() -> Vec<String> {
    // TODO (Techassi): Remove unwraps
    let client = KubeClient::new().await.unwrap();

    let namespaces = client.list_namespaces().await.unwrap();
    let mut ns = Vec::new();

    for namespace in namespaces {
        ns.push(namespace.metadata.name.unwrap_or("Unknown".into()))
    }

    ns
}
