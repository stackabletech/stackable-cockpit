use std::fmt::Display;

use kube::{ResourceExt, core::DynamicObject};
use serde::Serialize;
use snafu::{OptionExt, ResultExt, Snafu};

use crate::utils::k8s::{self, Client};

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("failed to fetch data from Kubernetes API"))]
    KubeClientFetch {
        #[snafu(source(from(k8s::Error, Box::new)))]
        source: Box<k8s::Error>,
    },

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
        write!(
            f,
            "{username}:{password}",
            username = self.username,
            password = self.password
        )
    }
}

/// Retrieves the credentials looking up a secret identified by `secret_name`
/// in `secret_namespace`. The function returns [`Ok(None)`] if `username_key`
/// and/or `password_key` are not found or the product does not provide
/// any credentials.
pub async fn get(
    client: &Client,
    product_name: &str,
    stacklet: &DynamicObject,
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
            let secret_name = stacklet.data["spec"]["clusterConfig"]["credentialsSecret"]
                .as_str()
                .context(NoSecretSnafu)?;

            client
                .get_credentials_from_secret(
                    secret_name,
                    &stacklet.namespace().unwrap(),
                    "adminUser.username",
                    "adminUser.password",
                )
                .await
                .context(KubeClientFetchSnafu)?
        }
        "nifi" => {
            let secret_name = stacklet.data["spec"]["clusterConfig"]["credentialsSecret"]
                .as_str()
                .context(NoSecretSnafu)?;

            client
                .get_credentials_from_secret(
                    secret_name,
                    &stacklet.namespace().unwrap(),
                    "username",
                    "password",
                )
                .await
                .context(KubeClientFetchSnafu)?
        }
        _ => return Ok(None),
    };

    Ok(Some(credentials))
}
