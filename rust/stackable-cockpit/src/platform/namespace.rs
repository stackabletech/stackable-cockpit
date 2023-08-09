use snafu::{ResultExt, Snafu};

use crate::kube::{KubeClient, KubeClientError};

#[derive(Debug, Snafu)]
pub enum NamespaceError {
    #[snafu(display("kubernetes client error"))]
    KubeClientError { source: KubeClientError },

    #[snafu(display("permission denied - try to create the namespace manually or choose an already existing one to which you have access to"))]
    PermissionDenied,
}

/// Creates a namespace with `name` if needed (not already present in the
/// cluster).
pub async fn create_if_needed(name: String) -> Result<(), NamespaceError> {
    let client = KubeClient::new().await.context(KubeClientSnafu)?;
    client
        .create_namespace_if_needed(name)
        .await
        .map_err(|err| match err {
            KubeClientError::KubeError { source } => match source {
                kube::Error::Api(err) if err.code == 401 => NamespaceError::PermissionDenied,
                _ => NamespaceError::KubeClientError {
                    source: KubeClientError::KubeError { source },
                },
            },
            _ => NamespaceError::KubeClientError { source: err },
        })
}
