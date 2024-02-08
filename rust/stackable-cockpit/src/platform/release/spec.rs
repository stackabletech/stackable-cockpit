use futures::{StreamExt as _, TryStreamExt};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};
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
