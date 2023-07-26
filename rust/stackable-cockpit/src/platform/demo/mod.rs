use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::common::{List, SpecIter};

mod spec;

pub use spec::*;

/// This struct describes a complete demos v2 file
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DemosV2 {
    #[serde(with = "serde_yaml::with::singleton_map_recursive")]
    demos: IndexMap<String, DemoSpecV2>,
}

impl SpecIter<DemoSpecV2> for DemosV2 {
    fn inner(&self) -> &IndexMap<String, DemoSpecV2> {
        &self.demos
    }
}

pub type DemoList = List<DemosV2, DemoSpecV2>;
