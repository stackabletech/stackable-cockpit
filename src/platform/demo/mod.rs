use std::{fs, path::PathBuf};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;

use crate::utils::{
    path::PathOrUrl,
    read::{read_cached_yaml_data, read_yaml_data, ReadError},
};

mod spec;

pub use spec::*;

/// This struct describes a complete demos v2 file
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DemosV2 {
    #[serde(with = "serde_yaml::with::singleton_map_recursive")]
    demos: IndexMap<String, DemoSpecV2>,
}

impl DemosV2 {
    pub fn inner(&self) -> &IndexMap<String, DemoSpecV2> {
        &self.demos
    }
}

#[derive(Debug)]
pub struct DemoList(IndexMap<String, DemoSpecV2>);

#[derive(Debug, Error)]
pub enum DemoListError {
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("read error: {0}")]
    ReadError(#[from] ReadError),

    #[error("url parse error: {0}")]
    ParseUrlError(#[from] url::ParseError),

    #[error("yaml error: {0}")]
    YamlError(#[from] serde_yaml::Error),
}

impl DemoList {
    pub async fn build<U, T>(
        remote_url: U,
        env_files: T,
        arg_files: T,
        use_cache: bool,
        cache_file_path: PathBuf,
    ) -> Result<Self, DemoListError>
    where
        U: AsRef<str>,
        T: AsRef<[PathOrUrl]>,
    {
        let mut map = IndexMap::new();
        let remote_url = Url::parse(remote_url.as_ref())?;

        // First load the remote demo file. This uses the cached file if present, and if not, requests the remote file
        // and then saves the contents on disk for cached use later
        let demos = if use_cache {
            match read_cached_yaml_data::<DemosV2>(cache_file_path.clone())? {
                Some(demos) => demos,
                None => {
                    let demos = read_yaml_data::<DemosV2>(remote_url).await?;
                    fs::write(cache_file_path, serde_yaml::to_string(&demos)?)?;
                    demos
                }
            }
        } else {
            read_yaml_data::<DemosV2>(remote_url).await?
        };

        for (demo_name, demo) in demos.inner() {
            map.insert(demo_name.to_owned(), demo.to_owned());
        }

        // After that, we load files provided via the ENV var
        for env_file in env_files.as_ref() {
            let demos = read_yaml_data::<DemosV2>(env_file).await?;
            for (demo_name, demo) in demos.inner() {
                map.insert(demo_name.to_owned(), demo.to_owned());
            }
        }

        // Lastly, the CLI argument --additional-demo-files is used
        for arg_file in arg_files.as_ref() {
            let demos = read_yaml_data::<DemosV2>(arg_file).await?;
            for (demo_name, demo) in demos.inner() {
                map.insert(demo_name.to_owned(), demo.to_owned());
            }
        }

        Ok(Self(map))
    }

    pub fn inner(&self) -> &IndexMap<String, DemoSpecV2> {
        &self.0
    }

    /// Get a demo by name
    pub fn get<T>(&self, demo_name: T) -> Option<&DemoSpecV2>
    where
        T: Into<String>,
    {
        self.0.get(&demo_name.into())
    }
}
