use std::{
    path::PathBuf,
    time::{Duration, SystemTime, SystemTimeError},
};

use snafu::{ResultExt, Snafu};
use tokio::{fs, io};

use crate::constants::DEFAULT_CACHE_MAX_AGE;

pub type CacheResult<T> = Result<T, CacheError>;

#[derive(Debug, Snafu)]
pub enum CacheError {
    #[snafu(display("io error"))]
    CacheIoError { source: io::Error },

    #[snafu(display("system time error"))]
    SystemTimeError { source: SystemTimeError },

    #[snafu(display("tried to write file with disabled cache"))]
    WriteDisabled,
}

#[derive(Debug)]
pub struct Cache {
    pub(crate) settings: CacheSettings,
}

impl Cache {
    /// Creates a new [`Cache`] instance with the provided `settings`. It should
    /// be noted that it is required to call the [`Cache::init`] method before
    /// using the cache for the first time to ensure the backend is setup
    /// properly.
    pub fn new(settings: CacheSettings) -> Self {
        Self { settings }
    }

    /// Initializes the cache backend. This ensure that local files and folders
    /// needed for operation are created.
    pub async fn init(&self) -> CacheResult<()> {
        match &self.settings.backend {
            CacheBackend::Disk { base_path } => {
                fs::create_dir_all(base_path).await.context(CacheIoSnafu)
            }
            CacheBackend::Disabled => Ok(()),
        }
    }

    pub fn is_enabled(&self) -> bool {
        match self.settings.backend {
            CacheBackend::Disk { .. } => true,
            CacheBackend::Disabled => false,
        }
    }

    pub async fn retrieve(&self, file_name: &str) -> CacheResult<CacheStatus<String>> {
        match &self.settings.backend {
            CacheBackend::Disk { base_path } => {
                let file_path = base_path.join(file_name);

                if !file_path.is_file() {
                    return Ok(CacheStatus::Miss);
                }

                let modified = file_path
                    .metadata()
                    .context(CacheIoSnafu {})?
                    .modified()
                    .context(CacheIoSnafu {})?;

                let elapsed = modified.elapsed().context(SystemTimeSnafu {})?;

                if elapsed > self.settings.max_age {
                    return Ok(CacheStatus::Expired);
                }

                let content = Self::read(file_path).await?;
                Ok(CacheStatus::Hit(content))
            }
            CacheBackend::Disabled => Ok(CacheStatus::Miss),
        }
    }

    pub async fn store(&self, file_name: &str, file_content: &str) -> CacheResult<()> {
        match &self.settings.backend {
            CacheBackend::Disk { base_path } => {
                let file_path = base_path.join(file_name);
                Self::write(file_path, file_content).await
            }
            CacheBackend::Disabled => WriteDisabledSnafu {}.fail(),
        }
    }

    pub async fn list(&self) -> CacheResult<Vec<(PathBuf, SystemTime)>> {
        match &self.settings.backend {
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

    pub async fn purge(&self) -> CacheResult<()> {
        match &self.settings.backend {
            CacheBackend::Disk { base_path } => {
                fs::remove_dir_all(base_path).await.context(CacheIoSnafu)?;
                fs::create_dir_all(base_path).await.context(CacheIoSnafu)
            }
            CacheBackend::Disabled => todo!(),
        }
    }

    async fn read(file_path: PathBuf) -> CacheResult<String> {
        fs::read_to_string(file_path).await.context(CacheIoSnafu {})
    }

    async fn write(file_path: PathBuf, file_content: &str) -> CacheResult<()> {
        fs::write(file_path, file_content)
            .await
            .context(CacheIoSnafu {})
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
