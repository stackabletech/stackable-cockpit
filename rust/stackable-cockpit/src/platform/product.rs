use semver::Version;
use serde::{Deserialize, Serialize};
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct ProductSpec {
    #[serde(rename = "operatorVersion")]
    #[cfg_attr(feature = "openapi", schema(value_type = String))]
    pub version: Version,
}
