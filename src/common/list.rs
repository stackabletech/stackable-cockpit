use std::{fs, marker::PhantomData};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};

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

#[derive(Debug, Snafu)]
pub enum ListError {
    #[snafu(display("io error: {source}"))]
    IoError { source: std::io::Error },

    #[snafu(display("local read error: {source}"))]
    LocalReadError { source: LocalReadError },

    #[snafu(display("remote read error: {source}"))]
    RemoteReadError { source: RemoteReadError },

    #[snafu(display("cached read error: {source}"))]
    CachedReadError { source: CachedReadError },

    #[snafu(display("url parse error: {source}"))]
    ParseUrlError { source: url::ParseError },

    #[snafu(display("yaml error: {source}"))]
    YamlError { source: serde_yaml::Error },

    #[snafu(display("invalid file url"))]
    InvalidFileUrl,
}

/// A [`List`] describes a list of specs. The list can contain any specs, for example demos, stacks or releases. The
/// generic parameter `L` represents the initial type of the spec list, directly deserialized from YAML. This type has
/// to implement [`SpecIter`], which returns a map of specs of type `S`.
#[derive(Debug, Serialize)]
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
    pub async fn build(
        files: &[PathOrUrl],
        cache_settings: CacheSettings,
    ) -> Result<Self, ListError> {
        let mut map = IndexMap::new();

        for file in files.as_ref() {
            let specs = match file {
                PathOrUrl::Path(path) => {
                    read_yaml_data_from_file::<L>(path.clone()).context(LocalReadSnafu {})?
                }
                PathOrUrl::Url(url) => {
                    if cache_settings.use_cache {
                        let file_name = url
                            .path_segments()
                            .ok_or(ListError::InvalidFileUrl)?
                            .last()
                            .ok_or(ListError::InvalidFileUrl)?;

                        let file_path = cache_settings.base_path.join(file_name);

                        match read_cached_yaml_data::<L>(file_path.clone(), &cache_settings)
                            .context(CachedReadSnafu {})?
                        {
                            CacheStatus::Hit(specs) => specs,
                            CacheStatus::Expired | CacheStatus::Miss => {
                                let data = read_yaml_data_from_remote::<L>(url.clone())
                                    .await
                                    .context(RemoteReadSnafu {})?;

                                let yaml = serde_yaml::to_string(&data).context(YamlSnafu {})?;
                                fs::write(file_path, yaml).context(IoSnafu {})?;

                                data
                            }
                        }
                    } else {
                        read_yaml_data_from_remote::<L>(url.clone())
                            .await
                            .context(RemoteReadSnafu {})?
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
