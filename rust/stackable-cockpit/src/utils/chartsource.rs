use std::collections::HashMap;

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct ChartSourceMetadata {
    pub entries: HashMap<String, Vec<ChartSourceEntry>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ChartSourceEntry {
    pub name: String,
    pub version: String,
}
