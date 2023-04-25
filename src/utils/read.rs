use std::{fs, io, path::PathBuf};

use serde::Deserialize;
use thiserror::Error;

use crate::utils::path::{IntoPathOrUrl, PathOrUrl, PathOrUrlParseError};

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
}

/// Reads YAML data from a remote URL or a local file and deserializes it into type `T`
pub async fn read_yaml_data<T>(path_or_url: impl IntoPathOrUrl) -> Result<T, ReadError>
where
    T: for<'a> Deserialize<'a> + Sized,
{
    let path_or_url = path_or_url.into_path_or_url()?;
    let data = read_from_file_or_url(path_or_url).await?;
    let yaml = serde_yaml::from_str::<T>(&data)?;

    Ok(yaml)
}

/// Reads potentially cached YAML data from a local file and deserializes it into type `T`. The function checks if the
/// provided path exists and is a file, and if yes, reads from this file. If the path doesn't exist or doesn't point
/// to a file, [`None`] is returned. A [`ReadError`] is returned when the file cannot be read or deserialization failed.
pub fn read_cached_yaml_data<T>(path: PathBuf) -> Result<Option<T>, ReadError>
where
    T: for<'a> Deserialize<'a> + Sized,
{
    if path.is_file() {
        let data = fs::read_to_string(path)?;
        let yaml = serde_yaml::from_str::<T>(&data)?;

        return Ok(Some(yaml));
    }

    Ok(None)
}

/// Reads the contents of a file either by retrieving a file via HTTP(S) or by reading a local file on disk via it's
/// file path.
pub async fn read_from_file_or_url(path_or_url: PathOrUrl) -> Result<String, ReadError> {
    match path_or_url {
        PathOrUrl::Path(path) => Ok(fs::read_to_string(path)?),
        PathOrUrl::Url(url) => Ok(reqwest::get(url).await?.text().await?),
    }
}
