use std::{
    collections::HashMap,
    fs, io,
    path::PathBuf,
    time::{Duration, SystemTimeError},
};

use serde::Deserialize;
use snafu::{ResultExt, Snafu};
use url::Url;

use crate::{
    constants::DEFAULT_CACHE_MAX_AGE,
    utils::{
        path::{IntoPathOrUrl, PathOrUrl, PathOrUrlParseError},
        templating,
    },
};

#[derive(Debug, Snafu)]
pub enum TemplatedReadError {
    #[snafu(display("local read error: {source}"))]
    TemplatedReadError { source: io::Error },

    #[snafu(display("request error: {source}"))]
    TemplatedRequestError { source: reqwest::Error },

    #[snafu(display("templating error: {source}"))]
    TemplatingError { source: tera::Error },

    #[snafu(display("yaml error: {source}"))]
    YamlError { source: serde_yaml::Error },

    #[snafu(display("path/url parse error: {source}"))]
    PathOrUrlParseError { source: PathOrUrlParseError },
}

#[derive(Debug, Snafu)]
pub enum CachedReadError {
    #[snafu(display("io error: {source}"))]
    CacheIoError { source: io::Error },

    #[snafu(display("system time error: {source}"))]
    SystemTimeError { source: SystemTimeError },

    #[snafu(display("local read error: {source}"))]
    LocalReadError { source: LocalReadError },
}

#[derive(Debug, Snafu)]
pub enum RemoteReadError {
    #[snafu(display("request error: {source}"))]
    RequestError { source: reqwest::Error },

    #[snafu(display("yaml parse error: {source}"))]
    RemoteYamlError { source: serde_yaml::Error },
}

#[derive(Debug, Snafu)]
pub enum LocalReadError {
    #[snafu(display("io error: {source}"))]
    LocalIoError { source: io::Error },

    #[snafu(display("yaml parse error: {source}"))]
    LocalYamlError { source: serde_yaml::Error },
}

/// Reads YAML data from a local file at `path` and deserializes it into type
/// `T`. A [`LocalReadError`] is returned when the file cannot be read or
/// deserialization failed.
pub fn read_yaml_data_from_file<T>(path: PathBuf) -> Result<T, LocalReadError>
where
    T: for<'a> Deserialize<'a> + Sized,
{
    let content = fs::read_to_string(path).context(LocalIoSnafu {})?;
    let data = serde_yaml::from_str(&content).context(LocalYamlSnafu {})?;

    Ok(data)
}

/// Reads YAML data from a remote file at `url` and deserializes it into type
/// `T`. A [`RemoteReadError`] is returned when the file cannot be read or
/// deserialization failed.
pub async fn read_yaml_data_from_remote<T>(url: Url) -> Result<T, RemoteReadError>
where
    T: for<'a> Deserialize<'a> + Sized,
{
    let content = reqwest::get(url)
        .await
        .context(RequestSnafu {})?
        .text()
        .await
        .context(RequestSnafu {})?;

    let data = serde_yaml::from_str(&content).context(RemoteYamlSnafu {})?;
    Ok(data)
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

/// Reads potentially cached YAML data from a local file and deserializes it
/// into type `T`. The function checks if the provided path exists and is a
/// file, and if yes, reads from this file. If the cache file exists,
/// [`CacheStatus::Hit`] is returned. If the path doesn't exist or doesn't
/// point to a file, [`CacheStatus::Miss`] is returned. If the cached file is
/// older then the provided max age, [`CacheStatus::Expired`] is returned. A
/// [`CachedReadError`] is returned when the file cannot be read or
/// deserialization failed.
pub fn read_cached_yaml_data<T>(
    path: PathBuf,
    settings: &CacheSettings,
) -> Result<CacheStatus<T>, CachedReadError>
where
    T: for<'a> Deserialize<'a> + Sized,
{
    if path.is_file() {
        let modified = path
            .metadata()
            .context(CacheIoSnafu {})?
            .modified()
            .context(CacheIoSnafu {})?;

        let elapsed = modified.elapsed().context(SystemTimeSnafu {})?;

        if elapsed > settings.max_age {
            return Ok(CacheStatus::Expired);
        }

        let data = read_yaml_data_from_file(path).context(LocalReadSnafu {})?;
        return Ok(CacheStatus::Hit(data));
    }

    Ok(CacheStatus::Miss)
}

/// Reads a local or remote (YAML) file and inserts parameter values based
/// on templating expressions. The parameters are passed into this function as
/// a [`HashMap<String, String>`]. The final templated result is returned as
/// a [`String`]. A [`TemplatedReadError`] is returned when the file cannot be
/// read, deserialization failed or the templating resulted in an error.
pub async fn read_yaml_data_with_templating<T, P>(
    path_or_url: P,
    parameters: &HashMap<String, String>,
) -> Result<T, TemplatedReadError>
where
    T: for<'a> Deserialize<'a>,
    P: IntoPathOrUrl,
{
    match path_or_url
        .into_path_or_url()
        .context(PathOrUrlParseSnafu {})?
    {
        PathOrUrl::Path(path) => read_yaml_data_from_file_with_templating(path, parameters),
        PathOrUrl::Url(url) => read_yaml_data_from_remote_with_templating(url, parameters).await,
    }
}

/// Reads YAML data from a local file at `path` and deserializes it into type
/// `T`. It also inserts parameter values based on templating expressions. The
/// parameters are passed into this function as a [`HashMap<String, String>`].
/// A [`TemplatedReadError`] is returned when the file cannot be read,
/// deserialization failed or the templating resulted in an error.
pub fn read_yaml_data_from_file_with_templating<T>(
    path: PathBuf,
    parameters: &HashMap<String, String>,
) -> Result<T, TemplatedReadError>
where
    T: for<'a> Deserialize<'a>,
{
    let content = fs::read_to_string(path).context(TemplatedReadSnafu {})?;

    // Render template
    let result = templating::render(&content, parameters).context(TemplatingSnafu {})?;
    serde_yaml::from_str(&result).context(YamlSnafu {})
}

/// Reads YAML data from a remote file at `url` and deserializes it into type
/// `T`. It also inserts parameter values based on templating expressions. The
/// parameters are passed into this function as a [`HashMap<String, String>`].
/// A [`TemplatedReadError`] is returned when the file cannot be read,
/// deserialization failed or the templating resulted in an error.
pub async fn read_yaml_data_from_remote_with_templating<T>(
    url: Url,
    parameters: &HashMap<String, String>,
) -> Result<T, TemplatedReadError>
where
    T: for<'a> Deserialize<'a>,
{
    let content = reqwest::get(url)
        .await
        .context(TemplatedRequestSnafu {})?
        .text()
        .await
        .context(TemplatedRequestSnafu {})?;

    // Render template
    let result = templating::render(&content, parameters).context(TemplatingSnafu {})?;
    serde_yaml::from_str(&result).context(YamlSnafu {})
}
