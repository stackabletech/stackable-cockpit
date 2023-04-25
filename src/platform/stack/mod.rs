use std::collections::HashMap;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{debug, info, instrument};
use url::Url;

mod spec;
pub use spec::*;

use crate::{
    common::ManifestSpec,
    utils::{
        params::{IntoParameters, IntoParametersError},
        path::PathOrUrl,
        read::{read_yaml_data, ReadError},
    },
};

/// This struct describes a complete demos v2 file
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StacksV2 {
    #[serde(with = "serde_yaml::with::singleton_map_recursive")]
    stacks: IndexMap<String, StackSpecV2>,
}

impl StacksV2 {
    pub fn inner(&self) -> &IndexMap<String, StackSpecV2> {
        &self.stacks
    }
}

pub struct StackList(IndexMap<String, StackSpecV2>);

#[derive(Debug, Error)]
pub enum StackListError {
    #[error("read error: {0}")]
    ReadError(#[from] ReadError),

    #[error("url parse error: {0}")]
    ParseUrlError(#[from] url::ParseError),
}

impl StackList {
    pub async fn build<U, T>(
        remote_url: U,
        env_files: T,
        arg_files: T,
    ) -> Result<Self, StackListError>
    where
        U: AsRef<str>,
        T: AsRef<[PathOrUrl]>,
    {
        let mut map = IndexMap::new();
        let remote_url = Url::parse(remote_url.as_ref())?;

        // First load the remote stack file
        let stacks = read_yaml_data::<StacksV2>(remote_url).await?;
        for (stack_name, stack) in stacks.inner() {
            map.insert(stack_name.to_owned(), stack.to_owned());
        }

        // After that, we load files provided via the ENV variable
        for env_file in env_files.as_ref() {
            let stacks = read_yaml_data::<StacksV2>(env_file).await?;
            for (stack_name, stack) in stacks.inner() {
                map.insert(stack_name.to_owned(), stack.to_owned());
            }
        }

        // Lastly, the CLI argument --additional-demo-files is used
        for arg_file in arg_files.as_ref() {
            let stacks = read_yaml_data::<StacksV2>(arg_file).await?;
            for (stack_name, stack) in stacks.inner() {
                map.insert(stack_name.to_owned(), stack.to_owned());
            }
        }

        Ok(Self(map))
    }

    pub fn inner(&self) -> &IndexMap<String, StackSpecV2> {
        &self.0
    }

    /// Get a demo by name
    pub fn get<T>(&self, stack_name: T) -> Option<&StackSpecV2>
    where
        T: Into<String>,
    {
        self.0.get(&stack_name.into())
    }
}

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
