use std::{fs, io};

use stackable::types::{PathOrUrl, PathOrUrlParseError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadError {
    #[error("io error: {0}")]
    IoError(#[from] io::Error),

    #[error("request error: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("parse error: {0}")]
    ParseError(#[from] PathOrUrlParseError),
}

/// Reads the contents of a file either by retrieving a file via HTTP(S) or by reading a local file on disk via it's
/// file path. The function checks if the provided `path_or_url` argument starts with `https` or `http`, which results
/// in a network request. Otherwise, it is assumed to be a file path.
pub async fn read_from_file_or_url<T>(path_or_url: T) -> Result<String, ReadError>
where
    T: TryInto<PathOrUrl>,
    ReadError: From<<T as TryInto<PathOrUrl>>::Error>, // It seems weird that we need this??
{
    let path_or_url = path_or_url.try_into()?;

    match path_or_url {
        PathOrUrl::Path(path) => Ok(fs::read_to_string(path)?),
        PathOrUrl::Url(url) => Ok(reqwest::get(url).await?.text().await?),
    }
}
