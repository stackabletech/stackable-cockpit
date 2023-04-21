use std::{path::PathBuf, str::FromStr};

use thiserror::Error;
use url::{ParseError, Url};

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

impl TryFrom<PathBuf> for PathOrUrl {
    type Error = PathOrUrlParseError;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        Ok(Self::Path(value))
    }
}

impl TryFrom<Url> for PathOrUrl {
    type Error = PathOrUrlParseError;

    fn try_from(value: Url) -> Result<Self, Self::Error> {
        Ok(Self::Url(value))
    }
}
