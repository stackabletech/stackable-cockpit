use std::{
    path::{Path, PathBuf},
    time::{Duration, SystemTime, SystemTimeError},
};

use sha2::{Digest, Sha256};
use snafu::{ResultExt, Snafu};
use tokio::{fs, io};
use url::Url;

use crate::constants::DEFAULT_CACHE_MAX_AGE;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("io error"))]
    CacheIoError { source: io::Error },

    #[snafu(display("system time error"))]
    SystemTimeError { source: SystemTimeError },

    #[snafu(display("tried to write file with disabled cache"))]
    WriteDisabled,
}

pub struct CacheBuilder {
    backend: CacheBackend,
    max_age: Duration,
}

impl Default for CacheBuilder {
    fn default() -> Self {
        Self {
            backend: CacheBackend::Disabled,
            max_age: DEFAULT_CACHE_MAX_AGE,
        }
    }
}

impl CacheBuilder {
    /// Sets the [`CacheBackend`] which should be used by the [`Cache`].
    /// Defaults to [`CacheBackend::Disabled`].
    pub fn with_backend(mut self, backend: CacheBackend) -> Self {
        self.backend = backend;
        self
    }

    /// Sets the cache max age. Defaults to [`DEFAULT_CACHE_MAX_AGE`].
    pub fn with_max_age(mut self, max_age: Duration) -> Self {
        self.max_age = max_age;
        self
    }

    ///  Creates a new [`Cache`] instance with the provided `settings`. It also
    /// initializes the cache backend. This ensure that local files and folders
    /// needed for operation are created.
    pub async fn build(self) -> Result<Cache> {
        match &self.backend {
            CacheBackend::Disk { base_path } => {
                fs::create_dir_all(base_path).await.context(CacheIoSnafu)?;
                Ok(Cache::new(self.backend, self.max_age))
            }
            CacheBackend::Disabled => todo!(),
        }
    }
}

#[derive(Debug)]
pub struct Cache {
    pub(crate) backend: CacheBackend,
    pub(crate) max_age: Duration,
}

impl Cache {
    /// Returns a [`CacheBuilder`] which allows to safely build and initialize
    /// a local cache.
    pub fn builder() -> CacheBuilder {
        CacheBuilder::default()
    }

    /// Returns wether the cache is enabled.
    pub fn is_enabled(&self) -> bool {
        match self.backend {
            CacheBackend::Disk { .. } => true,
            CacheBackend::Disabled => false,
        }
    }

    /// Retrieves cached content located at `file_name`. It should be noted that
    /// the `file_name` should only contain the file name and extension without
    /// any path segments prefixed. The cache internally makes sure the file is
    /// read from within the cache base path. The status is indicated by
    /// [`CacheStatus`]. An error is returned when the cache was unable to read
    /// data from disk.
    pub async fn retrieve(&self, file_url: &Url) -> Result<CacheStatus<String>> {
        match &self.backend {
            CacheBackend::Disk { base_path } => {
                let file_path = Self::file_path(base_path, file_url);

                if !file_path.is_file() {
                    return Ok(CacheStatus::Miss);
                }

                let modified = file_path
                    .metadata()
                    .context(CacheIoSnafu {})?
                    .modified()
                    .context(CacheIoSnafu {})?;

                let elapsed = modified.elapsed().context(SystemTimeSnafu {})?;

                if elapsed > self.max_age {
                    return Ok(CacheStatus::Expired);
                }

                let content = Self::read(file_path).await?;
                Ok(CacheStatus::Hit(content))
            }
            CacheBackend::Disabled => Ok(CacheStatus::Miss),
        }
    }

    /// Stores `file_content` at the cache base path in a file named `file_name`.
    /// The method returns an error if the cache fails to write the data to disk
    /// or the cache is disabled.
    pub async fn store(&self, file_url: &Url, file_content: &str) -> Result<()> {
        match &self.backend {
            CacheBackend::Disk { base_path } => {
                let file_path = Self::file_path(base_path, file_url);
                Self::write(file_path, file_content).await
            }
            CacheBackend::Disabled => WriteDisabledSnafu {}.fail(),
        }
    }

    /// Returns a list of currently cached files. This method makes no assumptions
    /// if the cached files are expired. It simply returns a list of files known
    /// by the cache.
    pub async fn list(&self) -> Result<Vec<(PathBuf, SystemTime)>> {
        match &self.backend {
            CacheBackend::Disk { base_path } => {
                let mut files = Vec::new();

                let mut entries = fs::read_dir(base_path).await.context(CacheIoSnafu)?;

                while let Some(entry) = entries.next_entry().await.context(CacheIoSnafu)? {
                    files.push((
                        entry.path(),
                        entry
                            .metadata()
                            .await
                            .context(CacheIoSnafu)?
                            .modified()
                            .context(CacheIoSnafu)?,
                    ))
                }

                files.sort();
                Ok(files)
            }
            CacheBackend::Disabled => Ok(vec![]),
        }
    }

    /// Removes all cached files by deleting the base cache folder and then
    /// recreating it.
    pub async fn purge(&self) -> Result<()> {
        match &self.backend {
            CacheBackend::Disk { base_path } => {
                fs::remove_dir_all(base_path).await.context(CacheIoSnafu)?;
                fs::create_dir_all(base_path).await.context(CacheIoSnafu)
            }
            CacheBackend::Disabled => todo!(),
        }
    }

    fn new(backend: CacheBackend, max_age: Duration) -> Self {
        Self { backend, max_age }
    }

    async fn read(file_path: PathBuf) -> Result<String> {
        fs::read_to_string(file_path).await.context(CacheIoSnafu {})
    }

    async fn write(file_path: PathBuf, file_content: &str) -> Result<()> {
        fs::write(file_path, file_content)
            .await
            .context(CacheIoSnafu {})
    }

    fn file_path(base_path: &Path, file_url: &Url) -> PathBuf {
        let mut hasher = Sha256::new();

        let sanitized_file_name = file_url
            .as_str()
            .replace(|c: char| !c.is_alphanumeric(), "-");

        hasher.update(file_url.as_str().as_bytes());
        let file_url_hash = hasher.finalize();

        base_path.join(format!("{sanitized_file_name}-{file_url_hash:x}"))
    }
}

pub enum CacheStatus<T> {
    Hit(T),
    Expired,
    Miss,
}

#[derive(Debug, Clone)]
pub struct CacheSettings {
    pub backend: CacheBackend,
    pub max_age: Duration,
}

impl From<CacheBackend> for CacheSettings {
    fn from(backend: CacheBackend) -> Self {
        Self {
            max_age: DEFAULT_CACHE_MAX_AGE,
            backend,
        }
    }
}

impl CacheSettings {
    pub fn disk(base_path: impl Into<PathBuf>) -> Self {
        CacheBackend::Disk {
            base_path: base_path.into(),
        }
        .into()
    }

    pub fn disabled() -> Self {
        CacheBackend::Disabled.into()
    }
}

#[derive(Debug, Clone)]
pub enum CacheBackend {
    Disk { base_path: PathBuf },
    Disabled,
}
