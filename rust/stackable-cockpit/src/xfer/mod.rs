use std::path::PathBuf;

use snafu::{ResultExt, Snafu};
use tokio::fs;
use url::Url;

pub mod cache;
pub mod processor;

use crate::{
    utils::path::PathOrUrl,
    xfer::{
        cache::{Cache, Settings, Status},
        processor::{Processor, ProcessorError},
    },
};

type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("failed to read local file"))]
    ReadLocalFile { source: std::io::Error },

    #[snafu(display("failed to create cache from provided settings"))]
    CacheSettings { source: cache::Error },

    #[snafu(display("failed to store file in cache"))]
    CacheStore { source: cache::Error },

    #[snafu(display("failed to retrieve file from cache"))]
    CacheRetrieve { source: cache::Error },

    #[snafu(display("failed to build http request"))]
    BuildRequest { source: reqwest::Error },

    #[snafu(display("failed to retrieve remote file contents"))]
    FetchRemoteContent { source: reqwest::Error },

    #[snafu(display("failed to process file contents"))]
    ProcessFileContent { source: ProcessorError },
}

#[derive(Debug)]
pub struct Client {
    pub(crate) client: reqwest::Client,
    pub(crate) cache: Cache,
}

impl Client {
    /// Creates a new [`Client`] with caching capabilities.
    pub async fn new(cache_settings: Settings) -> Result<Self> {
        let cache = cache_settings
            .try_into_cache()
            .await
            .context(CacheSettingsSnafu)?;
        let client = reqwest::Client::new();

        Ok(Self { client, cache })
    }

    pub fn new_with(cache: Cache) -> Self {
        let client = reqwest::Client::new();
        Self { client, cache }
    }

    /// Retrieves data from `path_or_url` which can either be a [`PathBuf`]
    /// or a [`Url`]. The `processor` defines how the data is processed, for
    /// example as plain text data, YAML content or even templated.
    pub async fn get<P>(&self, path_or_url: &PathOrUrl, processor: &P) -> Result<P::Output>
    where
        P: Processor<Input = String>,
    {
        match path_or_url {
            PathOrUrl::Path(path) => processor
                .process(self.get_from_local_file(path).await?)
                .context(ProcessFileContentSnafu),
            PathOrUrl::Url(url) => processor
                .process(self.get_from_cache_or_remote(url).await?)
                .context(ProcessFileContentSnafu),
        }
    }

    async fn get_from_local_file(&self, path: &PathBuf) -> Result<String> {
        fs::read_to_string(path).await.context(ReadLocalFileSnafu)
    }

    /// Internal method which either looks up the requested file in the cache
    /// or retrieves it from the remote located at `url` when the cache missed
    /// or is expired.
    async fn get_from_cache_or_remote(&self, url: &Url) -> Result<String> {
        match self.cache.retrieve(url).await.context(CacheRetrieveSnafu)? {
            Status::Hit(content) => Ok(content),
            Status::Expired | Status::Miss => {
                let content = self.get_from_remote(url).await?;
                self.cache
                    .store(url, &content)
                    .await
                    .context(CacheStoreSnafu)?;

                Ok(content)
            }
        }
    }

    /// Internal call which executes a HTTP GET request to `url`.
    async fn get_from_remote(&self, url: &Url) -> Result<String> {
        let req = self
            .client
            .get(url.clone())
            .build()
            .context(BuildRequestSnafu)?;

        let result = self
            .client
            .execute(req)
            .await
            .context(FetchRemoteContentSnafu)?;

        result.text().await.context(FetchRemoteContentSnafu)
    }
}
