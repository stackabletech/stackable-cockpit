use serde::{Deserialize, Serialize};

use crate::{
    common::ManifestSpec,
    utils::params::{Parameter, RawParameter, RawParameterParseError},
};

pub type RawDemoParameterParseError = RawParameterParseError;
pub type RawDemoParameter = RawParameter;
pub type DemoParameter = Parameter;

/// This struct describes a demo with the v2 spec
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DemoSpecV2 {
    /// A short description of the demo
    pub description: String,

    /// An optional link to a documentation page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,

    /// The name of the underlying stack
    #[serde(rename = "stackableStack")]
    pub stack: String,

    /// A variable number of labels (tags)
    #[serde(default)]
    pub labels: Vec<String>,

    /// A variable number of Helm or YAML manifests
    #[serde(default)]
    pub manifests: Vec<ManifestSpec>,

    /// A variable number of supported parameters
    #[serde(default)]
    pub parameters: Vec<DemoParameter>,
}
