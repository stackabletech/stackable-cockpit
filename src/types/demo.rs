use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::types::manifest::ManifestSpec;

/// This struct describes a complete demos v2 file
#[derive(Debug, Deserialize, Serialize)]
pub struct DemosV2 {
    demos: HashMap<String, DemoSpecV2>,
}

impl DemosV2 {
    pub fn iter(&self) -> std::collections::hash_map::Iter<String, DemoSpecV2> {
        self.demos.iter()
    }
}

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
}
