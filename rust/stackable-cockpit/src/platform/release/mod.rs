use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

mod spec;

pub use spec::*;

use crate::common::list::{List, SpecIter};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Releases {
    #[serde(with = "serde_yaml::with::singleton_map_recursive")]
    releases: IndexMap<String, ReleaseSpec>,
}

impl SpecIter<ReleaseSpec> for Releases {
    fn inner(&self) -> &IndexMap<String, ReleaseSpec> {
        &self.releases
    }
}

pub type ReleaseList = List<Releases, ReleaseSpec>;

#[derive(Default)]
pub struct Release {}
