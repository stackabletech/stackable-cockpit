use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ManifestSpec {
    HelmChart(String),
    PlainYaml(String),
}
