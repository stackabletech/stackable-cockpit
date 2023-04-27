use std::{fs, marker::PhantomData};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::utils::{
    path::PathOrUrl,
    read::{
        read_cached_yaml_data, read_yaml_data_from_file, read_yaml_data_from_remote, CacheSettings,
        CacheStatus, CachedReadError, LocalReadError, RemoteReadError,
    },
};

pub trait SpecIter<S> {
    fn inner(&self) -> &IndexMap<String, S>;
}

#[derive(Debug, Error)]
pub enum ListError {
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("local read error: {0}")]
    LocalReadError(#[from] LocalReadError),

    #[error("remote read error: {0}")]
    RemoteReadError(#[from] RemoteReadError),

    #[error("cached read error: {0}")]
    CachedReadError(#[from] CachedReadError),

    #[error("url parse error: {0}")]
    ParseUrlError(#[from] url::ParseError),

    #[error("yaml error: {0}")]
    YamlError(#[from] serde_yaml::Error),

    #[error("invalid file url")]
    InvalidFileUrl,
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
    pub async fn build<T>(files: T, cache_settings: CacheSettings) -> Result<Self, ListError>
    where
        T: AsRef<[PathOrUrl]>,
    {
        let mut map = IndexMap::new();

        for file in files.as_ref() {
            let specs = match file {
                PathOrUrl::Path(path) => read_yaml_data_from_file::<L>(path.clone())?,
                PathOrUrl::Url(url) => {
                    if cache_settings.use_cache {
                        let file_name = url
                            .path_segments()
                            .ok_or(ListError::InvalidFileUrl)?
                            .last()
                            .ok_or(ListError::InvalidFileUrl)?;

                        let file_path = cache_settings.base_path.join(file_name);

                        match read_cached_yaml_data::<L>(file_path.clone(), &cache_settings)? {
                            CacheStatus::Hit(specs) => specs,
                            CacheStatus::Expired | CacheStatus::Miss => {
                                let data = read_yaml_data_from_remote::<L>(url.clone()).await?;
                                let yaml = serde_yaml::to_string(&data)?;
                                fs::write(file_path, yaml)?;

                                data
                            }
                        }
                    } else {
                        read_yaml_data_from_remote::<L>(url.clone()).await?
                    }
                }
            };

            for (spec_name, spec) in specs.inner() {
                map.insert(spec_name.clone(), spec.clone());
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
}
