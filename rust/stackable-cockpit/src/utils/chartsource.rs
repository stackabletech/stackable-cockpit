use std::collections::HashMap;

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct ChartSourceMetadata {
    // TODO (@NickLarsenNZ): Add a field for the repo name.
    // This would remove the need for the nested HashMaps
    // See: around rust/stackablectl/src/cmds/operator.rs:519
    pub entries: HashMap<String, Vec<ChartSourceEntry>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ChartSourceEntry {
    pub name: String,
    pub version: String,
}
