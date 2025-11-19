use std::{path::PathBuf, str::FromStr};

use snafu::{ResultExt, Snafu};
use url::{ParseError, Url};

#[derive(Debug, Clone)]
pub enum PathOrUrl {
    Path(PathBuf),
    Url(Url),
}

#[derive(Debug, Snafu)]
pub enum PathOrUrlParseError {
    #[snafu(display("failed to parse URL"))]
    UrlParse { source: ParseError },
}

pub trait IntoPathOrUrl: Sized {
    fn into_path_or_url(self) -> Result<PathOrUrl, PathOrUrlParseError>;
}

impl<T: AsRef<str>> IntoPathOrUrl for T {
    fn into_path_or_url(self) -> Result<PathOrUrl, PathOrUrlParseError> {
        PathOrUrl::from_str(self.as_ref())
    }
}

pub trait IntoPathsOrUrls: Sized {
    fn into_paths_or_urls(self) -> Result<Vec<PathOrUrl>, PathOrUrlParseError>;
}

impl<T: AsRef<str>> IntoPathsOrUrls for Vec<T> {
    fn into_paths_or_urls(self) -> Result<Vec<PathOrUrl>, PathOrUrlParseError> {
        let mut paths_or_urls = Vec::new();

        for item in self {
            let path_or_url = item.into_path_or_url()?;
            paths_or_urls.push(path_or_url)
        }

        Ok(paths_or_urls)
    }
}

pub trait ParsePathsOrUrls {
    fn parse_paths_or_urls(self) -> Result<Vec<PathOrUrl>, PathOrUrlParseError>;
}

impl<T: AsRef<str>> ParsePathsOrUrls for T {
    fn parse_paths_or_urls(self) -> Result<Vec<PathOrUrl>, PathOrUrlParseError> {
        let items: Vec<&str> = self.as_ref().split(' ').collect();
        let mut paths_or_urls = Vec::new();

        for item in items {
            let path_or_url = item.into_path_or_url()?;
            paths_or_urls.push(path_or_url);
        }

        Ok(paths_or_urls)
    }
}

impl FromStr for PathOrUrl {
    type Err = PathOrUrlParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("https://") || s.starts_with("http://") {
            let url = Url::parse(s).context(UrlParseSnafu)?;
            return Ok(Self::Url(url));
        }

        let path = PathBuf::from(s);
        Ok(Self::Path(path))
    }
}
