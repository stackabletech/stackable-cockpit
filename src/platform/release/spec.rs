use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};
use tracing::{info, instrument};

#[cfg(feature = "openapi")]
use utoipa::ToSchema;

use crate::{
    helm::{self, HelmError},
    platform::{
        operator::{OperatorSpec, OperatorSpecParseError},
        product::ProductSpec,
    },
};

#[derive(Debug, Snafu)]
pub enum ReleaseInstallError {
    #[snafu(display("failed to parse operator spec: {source}"))]
    OperatorSpecParseError { source: OperatorSpecParseError },

    #[snafu(display("failed with Helm error: {source}"))]
    HelmError { source: HelmError },
}

#[derive(Debug, Snafu)]
pub enum ReleaseUninstallError {
    #[snafu(display("failed with Helm error"))]
    HelmUninstallError { source: HelmError },
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
    pub products: IndexMap<String, ProductSpec>,
}

impl ReleaseSpec {
    /// Installs a release by installing individual operators.
    #[instrument(skip_all)]
    pub fn install(
        &self,
        include_products: &[String],
        exclude_products: &[String],
        namespace: &str,
    ) -> Result<(), ReleaseInstallError> {
        info!("Installing release");

        for (product_name, product) in self.filter_products(include_products, exclude_products) {
            info!("Installing product {}", product_name);

            // Create operator spec
            let operator = OperatorSpec::new(product_name, Some(product.version.clone()))
                .context(OperatorSpecParseSnafu {})?;

            // Install operator
            operator.install(namespace).context(HelmSnafu {})?
        }
        Ok(())
    }

    pub fn uninstall(&self, namespace: &str) -> Result<(), ReleaseUninstallError> {
        for (product_name, _) in &self.products {
            helm::uninstall_release(product_name, namespace, true)
                .context(HelmUninstallSnafu {})?;
        }

        Ok(())
    }

    /// Filters out products based on if they are included or excluded.
    pub fn filter_products(
        &self,
        include_products: &[String],
        exclude_products: &[String],
    ) -> Vec<(String, ProductSpec)> {
        self.products
            .iter()
            .filter(|(name, _)| include_products.is_empty() || include_products.contains(name))
            .filter(|(name, _)| !exclude_products.contains(name))
            .map(|(name, product)| (name.clone(), product.clone()))
            .collect()
    }
}
