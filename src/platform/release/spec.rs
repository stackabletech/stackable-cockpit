use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};
use tracing::{info, instrument};

#[cfg(feature = "openapi")]
use utoipa::ToSchema;

use crate::{
    helm::HelmError,
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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct ReleaseSpec {
    /// Date this released was released
    #[serde(rename = "releaseDate")]
    date: String,

    /// A short description of this release
    description: String,

    /// List of products and their version in this release
    products: IndexMap<String, ProductSpec>,
}

impl ReleaseSpec {
    #[instrument(skip_all)]
    pub fn install<T>(
        &self,
        include_products: &[String],
        exclude_products: &[String],
        namespace: T,
    ) -> Result<(), ReleaseInstallError>
    where
        T: AsRef<str>,
    {
        info!("Installing release");

        for (product_name, product) in &self.products {
            let included = include_products.is_empty() || include_products.contains(product_name);
            let excluded = exclude_products.contains(product_name);

            if included && !excluded {
                info!("Installing product {}", product_name);

                // Create operator spec
                let operator = OperatorSpec::new(product_name, Some(product.version.clone()))
                    .context(OperatorSpecParseSnafu {})?;

                // Install operator
                operator.install(&namespace).context(HelmSnafu {})?
            }
        }

        Ok(())
    }
}
