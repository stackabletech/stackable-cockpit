use std::collections::HashMap;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{debug, info, instrument};

mod spec;
pub use spec::*;

use crate::{
    common::{List, ManifestSpec, SpecIter},
    utils::params::{IntoParameters, IntoParametersError},
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

#[derive(Debug)]
pub struct Stack {
    parameters: HashMap<String, String>,
    manifests: Vec<ManifestSpec>,
    operators: Vec<String>,
    release: String,
}

#[derive(Debug, Error)]
pub enum StackError {
    #[error("parameter parse error: {0}")]
    ParameterError(#[from] IntoParametersError),
}

impl Stack {
    #[instrument(skip_all)]
    pub fn new_from_spec(spec: &StackSpecV2, parameters: &[String]) -> Result<Self, StackError> {
        debug!("Creating stack");
        let parameters = parameters.to_owned().into_params(&spec.parameters)?;

        Ok(Self {
            manifests: spec.manifests.clone(),
            operators: spec.operators.clone(),
            release: spec.release.clone(),
            parameters,
        })
    }

    #[instrument(skip_all)]
    pub fn install(&self) {
        info!("Installing stack");
        todo!()
    }

    #[instrument(skip_all)]
    pub fn install_manifests(&self, demo_parameters: &HashMap<String, String>) {
        info!("Installing stack manifests");
        todo!()
    }
}
