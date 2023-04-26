use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use thiserror::Error;

mod spec;
pub use spec::*;

use crate::{
    common::{List, SpecIter},
    utils::params::IntoParametersError,
};

/// This struct describes a complete demos v2 file
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StacksV2 {
    #[serde(with = "serde_yaml::with::singleton_map_recursive")]
    stacks: IndexMap<String, StackSpecV2>,
}

impl SpecIter<StackSpecV2> for StacksV2 {
    fn inner(&self) -> &IndexMap<String, StackSpecV2> {
        &self.stacks
    }
}

pub type StackList = List<StacksV2, StackSpecV2>;

#[derive(Debug, Error)]
pub enum StackError {
    #[error("parameter parse error: {0}")]
    ParameterError(#[from] IntoParametersError),

    #[error("no such stack")]
    NoSuchStack,
}
