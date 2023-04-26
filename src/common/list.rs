use std::{fs, marker::PhantomData, path::PathBuf};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;

use crate::utils::{
    path::PathOrUrl,
    read::{read_cached_yaml_data, read_yaml_data, CacheSettings, CacheStatus, ReadError},
};

pub trait SpecIter<S> {
    fn inner(&self) -> &IndexMap<String, S>;
}

#[derive(Debug, Error)]
pub enum ListError {
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("read error: {0}")]
    ReadError(#[from] ReadError),

    #[error("url parse error: {0}")]
    ParseUrlError(#[from] url::ParseError),

    #[error("yaml error: {0}")]
    YamlError(#[from] serde_yaml::Error),
}

/// A [`List`] describes a list of specs. The list can contain any specs, for example demos, stacks or releases. The
/// generic parameter `L` represents the initial type of the spec list, directly deserialized from YAML. This type has
/// to implement [`SpecIter`], which returns a map of specs of type `S`.
pub struct List<L, S>
where
    L: for<'a> Deserialize<'a> + Serialize + SpecIter<S>,
    S: for<'a> Deserialize<'a> + Serialize + Clone,
{
    pub inner: IndexMap<String, S>,
    list_type: PhantomData<L>,
}

impl<L, S> List<L, S>
where
    L: for<'a> Deserialize<'a> + Serialize + SpecIter<S>,
    S: for<'a> Deserialize<'a> + Serialize + Clone,
{
    pub async fn build<U, T>(
        remote_url: U,
        env_files: T,
        arg_files: T,
        cache_settings: CacheSettings,
    ) -> Result<Self, ListError>
    where
        U: AsRef<str>,
        T: AsRef<[PathOrUrl]>,
    {
        let mut map = IndexMap::new();
        let remote_url = Url::parse(remote_url.as_ref())?;

        // First load the remote demo file. This uses the cached file if present, and if not, requests the remote file
        // and then saves the contents on disk for cached use later
        for (spec_name, spec) in Self::get_default_file(remote_url, cache_settings)
            .await?
            .inner()
        {
            map.insert(spec_name.clone(), spec.clone());
        }

        // After that, we load files provided via the ENV var
        for env_file in env_files.as_ref() {
            let demos = read_yaml_data::<L>(env_file).await?;
            for (demo_name, demo) in demos.inner() {
                map.insert(demo_name.to_owned(), demo.to_owned());
            }
        }

        // Lastly, the CLI argument --additional-demo-files is used
        for arg_file in arg_files.as_ref() {
            let demos = read_yaml_data::<L>(arg_file).await?;
            for (demo_name, demo) in demos.inner() {
                map.insert(demo_name.to_owned(), demo.to_owned());
            }
        }

        Ok(Self {
            list_type: PhantomData,
            inner: map,
        })
    }

    /// Returns a reference to the inner [`IndexMap`]
    pub fn inner(&self) -> &IndexMap<String, S> {
        &self.inner
    }

    /// Returns an optional reference to a single spec of type `S` by `name`
    pub fn get<T>(&self, name: T) -> Option<&S>
    where
        T: Into<String>,
    {
        self.inner.get(&name.into())
    }

    async fn get_default_file(
        remote_url: Url,
        cache_settings: CacheSettings,
    ) -> Result<L, ListError> {
        let specs = if cache_settings.use_cache {
            match read_cached_yaml_data::<L>(&cache_settings)? {
                CacheStatus::Hit(demos) => demos,
                CacheStatus::Expired | CacheStatus::Miss => {
                    let demos = read_yaml_data::<L>(remote_url).await?;
                    fs::write(cache_settings.file_path, serde_yaml::to_string(&demos)?)?;
                    demos
                }
            }
        } else {
            read_yaml_data::<L>(remote_url).await?
        };

        Ok(specs)
    }
}
