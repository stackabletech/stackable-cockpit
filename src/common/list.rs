use std::{fs, marker::PhantomData, path::Path};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};

use crate::utils::{
    path::PathOrUrl,
    read::{
        read_cached_yaml_data, read_yaml_data_from_file, read_yaml_data_from_remote, CacheBackend,
        CacheSettings, CacheStatus, CachedReadError, LocalReadError, RemoteReadError,
    },
};

pub trait SpecIter<S> {
    fn inner(&self) -> &IndexMap<String, S>;
}

#[derive(Debug, Snafu)]
pub enum ListError {
    #[snafu(display("io error"))]
    IoError { source: std::io::Error },

    #[snafu(display("local read error"))]
    LocalReadError { source: LocalReadError },

    #[snafu(display("remote read error"))]
    RemoteReadError { source: RemoteReadError },

    #[snafu(display("cached read error"))]
    CachedReadError { source: CachedReadError },

    #[snafu(display("url parse error"))]
    ParseUrlError { source: url::ParseError },

    #[snafu(display("yaml error"))]
    YamlError { source: serde_yaml::Error },

    #[snafu(display("invalid file url"))]
    InvalidFileUrl,
}

/// A [`List`] describes a list of specs. The list can contain any specs, for
/// example demos, stacks or releases. The generic parameter `L` represents
/// the initial type of the spec list, directly deserialized from YAML. This
/// type has to implement [`SpecIter`], which returns a map of specs of type
/// `S`.
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
    /// Builds a list of specs of type `S` based on a list of files. These files
    /// can be located locally (on disk) or remotely. Remote files will get
    /// downloaded once and then will be cached locally for a specified amount
    /// of time. The cache time, cache base path and whether to use the cache at
    /// all are specified using the [`CacheSettings`].
    pub async fn build_raw(
        files: &[PathOrUrl],
        cache_settings: &CacheSettings,
    ) -> Result<Self, ListError> {
        let mut map = IndexMap::new();

        for file in files {
            let specs = match file {
                PathOrUrl::Path(path) => {
                    read_yaml_data_from_file::<L>(path.clone()).context(LocalReadSnafu {})?
                }
                PathOrUrl::Url(url) => match &cache_settings.backend {
                    CacheBackend::Disabled => read_yaml_data_from_remote::<L>(url.clone())
                        .await
                        .context(RemoteReadSnafu {})?,
                    CacheBackend::Disk {
                        base_path: cache_base_path,
                    } => {
                        let file_name = url
                            .path_segments()
                            .ok_or(ListError::InvalidFileUrl)?
                            .last()
                            .ok_or(ListError::InvalidFileUrl)?;

                        let file_path = cache_base_path.join(file_name);

                        match read_cached_yaml_data::<L>(file_path.clone(), cache_settings)
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
                    }
                },
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

    /// Builds a list of specs of type `S` based on a list of files. These files
    /// can be located locally (on disk) or remotely. Remote files will get
    /// downloaded once and then will be cached locally for a specified amount
    /// of time.
    ///
    /// `cache_dir_prefix` is a unique directory under `XDG_CACHE_HOME`.
    /// Files will get stored at `$HOME/.cache/stackablectl` for example.
    ///
    /// `use_cache` specifies if the cache should be used.
    ///
    /// This function is a shortcut and uses some predefined paths, durations
    /// and settings. Full control over the cache settings is available with
    /// [`Self::build_raw()`].
    pub async fn build<P>(
        files: &[PathOrUrl],
        cache_dir_prefix: P,
        use_cache: bool,
    ) -> Result<Self, ListError>
    where
        P: AsRef<Path>,
    {
        let cache_home_path = xdg::BaseDirectories::with_prefix(cache_dir_prefix)
            .context(XdgSnafu {})?
            .get_cache_home();

        Self::build_raw(
            files,
            CacheSettings::new_from_path_and_enabled(cache_home_path, use_cache),
        )
        .await
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
