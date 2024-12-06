use futures::{StreamExt as _, TryStreamExt};
use indexmap::IndexMap;
use regex::Regex;
use serde::{Deserialize, Serialize};
use snafu::{OptionExt, ResultExt, Snafu};
use tokio::task::JoinError;
use tracing::{info, instrument};

#[cfg(feature = "openapi")]
use utoipa::ToSchema;

use crate::{
    helm,
    platform::{
        operator::{self, OperatorSpec},
        product,
    },
};

use super::ReleaseList;

type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("failed to parse operator spec"))]
    OperatorSpecParse { source: operator::SpecParseError },

    #[snafu(display("failed to install release using Helm"))]
    HelmInstall { source: helm::Error },

    #[snafu(display("failed to uninstall release using Helm"))]
    HelmUninstall { source: helm::Error },

    #[snafu(display("failed to launch background task"))]
    BackgroundTask { source: JoinError },

    #[snafu(display("release list is empty"))]
    EmptyReleaseList,

    #[snafu(display("latest release doesn't have expected format"))]
    LatestReleaseFormat,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct ReleaseSpec {
    /// Date this released was released
    #[serde(rename = "releaseDate")]
    pub date: String,

    /// A short description of this release
    pub description: String,

    /// List of products and their version in this release
    pub products: IndexMap<String, product::ProductSpec>,
}

impl ReleaseSpec {
    /// Installs a release by installing individual operators.
    #[instrument(skip_all)]
    pub async fn install(
        &self,
        include_products: &[String],
        exclude_products: &[String],
        namespace: &str,
    ) -> Result<()> {
        info!("Installing release");

        let namespace = namespace.to_string();
        futures::stream::iter(self.filter_products(include_products, exclude_products))
            .map(|(product_name, product)| {
                let namespace = namespace.clone();
                // Helm installs currently `block_in_place`, so we need to spawn each job onto a separate task to
                // get useful parallelism.
                tokio::spawn(async move {
                    info!("Installing {product_name}-operator");

                    // Create operator spec
                    let operator = OperatorSpec::new(&product_name, Some(product.version.clone()))
                        .context(OperatorSpecParseSnafu)?;

                    // Install operator
                    operator.install(&namespace).context(HelmInstallSnafu)?;

                    info!("Installed {product_name}-operator");

                    Ok(())
                })
            })
            .buffer_unordered(10)
            .map(|res| res.context(BackgroundTaskSnafu)?)
            .try_collect::<()>()
            .await
    }

    #[instrument(skip_all)]
    pub fn uninstall(&self, namespace: &str) -> Result<()> {
        info!("Uninstalling release");

        for (product_name, product_spec) in &self.products {
            info!("Uninstalling {product_name}-operator");

            // Create operator spec
            let operator = OperatorSpec::new(product_name, Some(product_spec.version.clone()))
                .context(OperatorSpecParseSnafu)?;

            // Uninstall operator
            helm::uninstall_release(&operator.helm_name(), namespace, true)
                .context(HelmUninstallSnafu)?;
        }

        Ok(())
    }

    /// Filters out products based on if they are included or excluded.
    pub fn filter_products(
        &self,
        include_products: &[String],
        exclude_products: &[String],
    ) -> Vec<(String, product::ProductSpec)> {
        self.products
            .iter()
            .filter(|(name, _)| include_products.is_empty() || include_products.contains(name))
            .filter(|(name, _)| !exclude_products.contains(name))
            .map(|(name, product)| (name.clone(), product.clone()))
            .collect()
    }
}

impl ReleaseList {
    /// Checks if a value provided in the '--release' argument is in the release list
    pub fn contains(&self, release: &str) -> bool {
        self.inner().contains_key(release)
    }

    /// Retrieves the latest release from the list and applies a sanity check to the release format.
    pub fn latest_release(&self) -> Result<String, Error> {
        let release = self.inner().first().context(EmptyReleaseListSnafu)?.0;
        let sanity_check = Regex::new("^[0-9]{2}.[0-9]{1,2}$").unwrap();
        if sanity_check.is_match(release) {
            Ok(release.to_string())
        } else {
            LatestReleaseFormatSnafu {}.fail()
        }
    }
}
