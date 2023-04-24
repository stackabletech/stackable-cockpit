use std::{fs, io};

use serde::Deserialize;
use thiserror::Error;

use crate::types::{IntoPathOrUrl, PathOrUrl, PathOrUrlParseError};

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

/// Reads the contents of a file either by retrieving a file via HTTP(S) or by reading a local file on disk via it's
/// file path.
pub async fn read_from_file_or_url(path_or_url: PathOrUrl) -> Result<String, ReadError> {
    match path_or_url {
        PathOrUrl::Path(path) => Ok(fs::read_to_string(path)?),
        PathOrUrl::Url(url) => Ok(reqwest::get(url).await?.text().await?),
    }
}
