use std::fmt::Display;

use kube::{core::DynamicObject, ResourceExt};
use serde::Serialize;
use snafu::{ResultExt, Snafu};

use crate::utils::k8s::{KubeClient, KubeClientError};

pub type Result<T, E = CredentialsError> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum CredentialsError {
    #[snafu(display("kubernetes error"))]
    KubeError { source: KubeClientError },

    #[snafu(display("no credentials secret found"))]
    NoSecret,
}

#[derive(Debug, Serialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

impl Display for Credentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.username, self.password)
    }
}

/// Retrieves the credentials looking up a secret identified by `secret_name`
/// in `secret_namespace`. The function returns [`Ok(None)`] if `username_key`
/// and/or `password_key` are not found or the product does not provide
/// any credentials.
pub async fn get_credentials(
    kube_client: &KubeClient,
    product_name: &str,
    product_crd: &DynamicObject,
) -> Result<Option<Credentials>> {
    // FIXME (Techassi): This should be discoverable, instead of hard-coding
    // supported products. Additionally, all the username and password keys
    // should be the same to further simplify the implementation. This is
    // part of many upcoming changes unifying the SDP.

    // FIXME (Techassi): Add a separate CredentialsError which indicates what
    // went wrong when retrieving credentials:
    // - No permissions to do so, most likely a 403
    // - Secret not found, most likely a 404
    // - No credentialsSecret present, None below
    // - Username and/or password key not found, don't return Option

    let credentials = match product_name {
        "airflow" | "superset" => {
            match product_crd.data["spec"]["clusterConfig"]["credentialsSecret"].as_str() {
                Some(secret_name) => kube_client
                    .get_credentials_from_secret(
                        secret_name,
                        &product_crd.namespace().unwrap(),
                        "adminUser.username",
                        "adminUser.password",
                    )
                    .await
                    .context(KubeSnafu)?,
                None => return Err(NoSecretSnafu.build()),
            }
        }
        "nifi" => match product_crd.data["spec"]["clusterConfig"]["credentialsSecret"].as_str() {
            Some(secret_name) => kube_client
                .get_credentials_from_secret(
                    secret_name,
                    &product_crd.namespace().unwrap(),
                    "username",
                    "password",
                )
                .await
                .context(KubeSnafu)?,
            None => return Err(NoSecretSnafu.build()),
        },
        _ => return Ok(None),
    };

    Ok(Some(credentials))
}
