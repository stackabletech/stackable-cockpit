use std::{path::PathBuf, str::FromStr};

use thiserror::Error;
use url::{ParseError, Url};

#[derive(Debug, Clone)]
pub enum PathOrUrl {
    Path(PathBuf),
    Url(Url),
}

#[derive(Debug, Error)]
pub enum PathOrUrlParseError {
    #[error("url parse error: {0}")]
    UrlParseError(#[from] ParseError),
}

impl FromStr for PathOrUrl {
    type Err = PathOrUrlParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("https://") || s.starts_with("http://") {
            return Ok(Self::Url(Url::parse(s)?));
        }

        let path = PathBuf::from(s);
        Ok(Self::Path(path))
    }
}

impl TryFrom<String> for PathOrUrl {
    type Error = PathOrUrlParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(value.as_str())
    }
}

impl TryFrom<&str> for PathOrUrl {
    type Error = PathOrUrlParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl From<PathBuf> for PathOrUrl {
    fn from(value: PathBuf) -> Self {
        Self::Path(value)
    }
}

impl From<Url> for PathOrUrl {
    fn from(value: Url) -> Self {
        Self::Url(value)
    }
}

impl From<&PathOrUrl> for PathOrUrl {
    fn from(value: &PathOrUrl) -> Self {
        value.clone()
    }
}
