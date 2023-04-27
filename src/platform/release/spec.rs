use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};

use crate::platform::product::ProductSpec;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
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
    pub fn install(&self, include_products: &[String], exclude_products: &[String]) {
        info!("Installing release");

        for (product_name, product) in &self.products {
            let included = include_products.is_empty() || include_products.contains(&product_name);
            let excluded = exclude_products.contains(&product_name);

            if included && !excluded {
                // Install operator
                todo!()
            }
        }

        todo!()
    }
}
