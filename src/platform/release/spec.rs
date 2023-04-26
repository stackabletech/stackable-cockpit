use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

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
