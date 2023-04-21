use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;

mod spec;
pub use spec::*;

use crate::{
    types::PathOrUrl,
    utils::read::{read_yaml_data, ReadError},
};

/// This struct describes a complete demos v2 file
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StacksV2 {
    #[serde(with = "serde_yaml::with::singleton_map_recursive")]
    stacks: HashMap<String, StackSpecV2>,
}

impl StacksV2 {
    pub fn inner(&self) -> &HashMap<String, StackSpecV2> {
        &self.stacks
    }
}

pub struct StackList(HashMap<String, StackSpecV2>);

#[derive(Debug, Error)]
pub enum StackListError {
    #[error("read error: {0}")]
    ReadError(#[from] ReadError),
}

impl StackList {
    pub async fn build<U, T>(
        remote_url: U,
        env_files: T,
        arg_files: T,
    ) -> Result<Self, StackListError>
    where
        U: Into<Url>,
        T: AsRef<[PathOrUrl]>,
    {
        let mut map = HashMap::new();

        // First load the remote stack file
        let stacks = read_yaml_data::<StacksV2>(remote_url.into()).await?;
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
}
