use std::{
    fs, io,
    path::PathBuf,
    time::{Duration, SystemTimeError},
};

use serde::Deserialize;
use thiserror::Error;

use crate::{
    constants::DEFAULT_CACHE_MAX_AGE_IN_SECS,
    utils::path::{IntoPathOrUrl, PathOrUrl, PathOrUrlParseError},
};

#[derive(Debug, Error)]
pub enum ReadError {
    #[error("io error: {0}")]
    IoError(#[from] io::Error),

    #[error("request error: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("yaml parse error: {0}")]
    YamlError(#[from] serde_yaml::Error),

    #[error("path/url parse error: {0}")]
    PathOrUrlParseError(#[from] PathOrUrlParseError),

    #[error("system time error: {0}")]
    SystemTimeError(#[from] SystemTimeError),
}

/// Reads YAML data from a remote URL or a local file and deserializes it into type `T`. A [`ReadError`] is returned
/// when the file cannot be read or deserialization failed.
pub async fn read_yaml_data<T>(path_or_url: impl IntoPathOrUrl) -> Result<T, ReadError>
where
    T: for<'a> Deserialize<'a> + Sized,
{
    let path_or_url = path_or_url.into_path_or_url()?;
    let data = read_from_file_or_url(path_or_url).await?;
    let yaml = serde_yaml::from_str::<T>(&data)?;

    Ok(yaml)
}

pub enum CacheStatus<T> {
    Hit(T),
    Expired,
    Miss,
}

pub struct CacheSettings {
    pub file_path: PathBuf,
    pub max_age: Duration,
    pub use_cache: bool,
}

impl From<(PathBuf, Duration, bool)> for CacheSettings {
    fn from(value: (PathBuf, Duration, bool)) -> Self {
        Self {
            file_path: value.0,
            max_age: value.1,
            use_cache: value.2,
        }
    }
}

impl From<(PathBuf, Duration)> for CacheSettings {
    fn from(value: (PathBuf, Duration)) -> Self {
        Self {
            file_path: value.0,
            max_age: value.1,
            use_cache: true,
        }
    }
}

impl From<(PathBuf, bool)> for CacheSettings {
    fn from(value: (PathBuf, bool)) -> Self {
        Self {
            max_age: Duration::from_secs(DEFAULT_CACHE_MAX_AGE_IN_SECS),
            file_path: value.0,
            use_cache: value.1,
        }
    }
}

impl From<PathBuf> for CacheSettings {
    fn from(value: PathBuf) -> Self {
        Self {
            max_age: Duration::from_secs(DEFAULT_CACHE_MAX_AGE_IN_SECS),
            file_path: value,
            use_cache: true,
        }
    }
}

/// Reads potentially cached YAML data from a local file and deserializes it into type `T`. The function checks if the
/// provided path exists and is a file, and if yes, reads from this file. If the cache file exists, [`CacheStatus::Hit`]
/// is returned. If the path doesn't exist or doesn't point to a file, [`CacheStatus::Miss`] is returned. If the cached
/// file is older then the provided max age, [`CacheStatus::Expired`] is returned. A [`ReadError`] is returned when
/// the file cannot be read or deserialization failed.
pub fn read_cached_yaml_data<T>(settings: &CacheSettings) -> Result<CacheStatus<T>, ReadError>
where
    T: for<'a> Deserialize<'a> + Sized,
{
    if settings.file_path.is_file() {
        let modified = settings.file_path.metadata()?.modified()?;
        let elapsed = modified.elapsed()?;

        if elapsed > settings.max_age {
            return Ok(CacheStatus::Expired);
        }

        let data = fs::read_to_string(settings.file_path.clone())?;
        let yaml = serde_yaml::from_str::<T>(&data)?;

        return Ok(CacheStatus::Hit(yaml));
    }

    Ok(CacheStatus::Miss)
}

/// Reads the contents of a file either by retrieving a file via HTTP(S) or by reading a local file on disk via it's
/// file path. A [`ReadError`] is returned when the file cannot be read or deserialization failed.
pub async fn read_from_file_or_url(path_or_url: PathOrUrl) -> Result<String, ReadError> {
    match path_or_url {
        PathOrUrl::Path(path) => Ok(fs::read_to_string(path)?),
        PathOrUrl::Url(url) => Ok(reqwest::get(url).await?.text().await?),
    }
}
