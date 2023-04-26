use std::{fs, io, path::PathBuf, time::Duration};

use serde::Deserialize;
use thiserror::Error;
use url::Url;

use crate::{
    constants::DEFAULT_CACHE_MAX_AGE_IN_SECS,
    utils::path::{IntoPathOrUrl, PathOrUrl, PathOrUrlParseError},
};

#[derive(Debug, Error)]
pub enum ReadError {
    #[error("path/url parse error: {0}")]
    PathOrUrlParseError(#[from] PathOrUrlParseError),

    #[error("local read error: {0}")]
    LocalReadError(#[from] LocalReadError),

    #[error("remote read error: {0}")]
    RemoteReadError(#[from] RemoteReadError),
}

#[derive(Debug, Error)]
pub enum RemoteReadError {
    #[error("request error: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("yaml parse error: {0}")]
    YamlError(#[from] serde_yaml::Error),
}

#[derive(Debug, Error)]
pub enum LocalReadError {
    #[error("io error: {0}")]
    IoError(#[from] io::Error),

    #[error("yaml parse error: {0}")]
    YamlError(#[from] serde_yaml::Error),
}

/// Reads YAML data from a remote URL or a local file and deserializes it into type `T`. A [`ReadError`] is returned
/// when the file cannot be read or deserialization failed.
pub async fn read_yaml_data<T>(path_or_url: impl IntoPathOrUrl) -> Result<T, ReadError>
where
    T: for<'a> Deserialize<'a> + Sized,
{
    let data = match path_or_url.into_path_or_url()? {
        PathOrUrl::Path(path) => read_yaml_data_from_file(path)?,
        PathOrUrl::Url(url) => read_yaml_data_from_remote(url).await?,
    };

    Ok(data)
}

/// Reads YAML data from a local file at `path` and deserializes it into type `T`. A [`LocalReadError`] is returned
/// when the file cannot be read or deserialization failed.
pub fn read_yaml_data_from_file<T>(path: PathBuf) -> Result<T, LocalReadError>
where
    T: for<'a> Deserialize<'a> + Sized,
{
    let content = fs::read_to_string(path)?;
    let data = serde_yaml::from_str(&content)?;

    Ok(data)
}

/// Reads YAML data from a remote file at `url` and deserializes it into type `T`. A [`RemoteReadError`] is returned
/// when the file cannot be read or deserialization failed.
pub async fn read_yaml_data_from_remote<T>(url: Url) -> Result<T, RemoteReadError>
where
    T: for<'a> Deserialize<'a> + Sized,
{
    let content = reqwest::get(url).await?.text().await?;
    let data = serde_yaml::from_str(&content)?;

    Ok(data)
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
pub fn read_cached_yaml_data<T>(settings: &CacheSettings) -> Result<CacheStatus<T>, RemoteReadError>
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
