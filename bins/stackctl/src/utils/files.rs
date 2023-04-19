use std::{fs, io};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadError {
    #[error("io error: {0}")]
    IoError(#[from] io::Error),

    #[error("request error: {0}")]
    RequestError(#[from] reqwest::Error),
}

/// Reads the contents of a file either by retrieving a file via HTTP(S) or by reading a local file on disk via it's
/// file path. The function checks if the provided `path_or_url` argument starts with `https` or `http`, which results
/// in a network request. Otherwise, it is assumed to be a file path.
pub async fn read_from_file_or_url(path_or_url: &str) -> Result<String, ReadError> {
    if path_or_url.starts_with("https") || path_or_url.starts_with("http") {
        return Ok(reqwest::get(path_or_url).await?.text().await?);
    }

    Ok(fs::read_to_string(path_or_url)?)
}
