use snafu::{ensure, ResultExt, Snafu};
use url::Url;

mod cache;
pub use cache::*;

pub mod parser;

use self::parser::Parser;

type Result<T> = core::result::Result<T, TransferError>;

#[derive(Debug, Snafu)]
pub enum TransferError {
    #[snafu(display("failed to extract file name from URL"))]
    FileNameError,

    #[snafu(display("cache error"))]
    CacheError { source: CacheError },

    #[snafu(display("request error"))]
    RequestError { source: reqwest::Error },

    #[snafu(display("failed to deserialize content into YAML"))]
    YamlError { source: serde_yaml::Error },

    #[snafu(display("templating error"))]
    TemplatingError { source: tera::Error },
}

#[derive(Debug)]
pub struct TransferClient {
    pub(crate) client: reqwest::Client,
    pub(crate) cache: Cache,
}

impl TransferClient {
    /// Creates a new [`TransferClient`] with caching capabilities.
    pub fn new(cache_settings: CacheSettings) -> Self {
        let cache = Cache::new(cache_settings);
        let client = reqwest::Client::new();

        Self { client, cache }
    }

    pub async fn get<P>(&self, url: &Url, parser: &P) -> Result<P::Output>
    where
        P: Parser<Input = String>,
    {
        parser.parse(self.get_from_cache_or_remote(url).await?)
    }

    /// Internal method which either looks up the requested file in the cache
    /// or retrieves it from the remote located at `url` when the cache missed
    /// or is expired.
    async fn get_from_cache_or_remote(&self, url: &Url) -> Result<String> {
        let file_name = Self::extract_file_name(url)?;

        match self
            .cache
            .retrieve(&file_name)
            .await
            .context(CacheSnafu {})?
        {
            CacheStatus::Hit(content) => Ok(content),
            CacheStatus::Expired | CacheStatus::Miss => {
                let content = self.get_from_remote(url).await?;
                self.cache
                    .store(&file_name, &content)
                    .await
                    .context(CacheSnafu {})?;

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
            .context(RequestSnafu {})?;
        let result = self.client.execute(req).await.context(RequestSnafu {})?;

        result.text().await.context(RequestSnafu {})
    }

    /// Returns the last URL path segment as the filename. It ensures that the
    /// last segment contains at least one dot.
    fn extract_file_name(url: &Url) -> Result<String> {
        let segment = url
            .path_segments()
            .ok_or(TransferError::FileNameError)?
            .last()
            .ok_or(TransferError::FileNameError)?;

        ensure!(segment.contains('.'), FileNameSnafu {});
        Ok(segment.to_string())
    }
}
