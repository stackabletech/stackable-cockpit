use std::{
    path::PathBuf,
    time::{Duration, SystemTimeError},
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

pub struct Cache {
    pub(crate) settings: CacheSettings,
}

impl Cache {
    pub fn new(settings: CacheSettings) -> Self {
        Self { settings }
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

pub enum CacheBackend {
    Disk { base_path: PathBuf },
    Disabled,
}
