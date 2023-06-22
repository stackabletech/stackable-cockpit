use serde::Deserialize;
use snafu::{ensure, ResultExt, Snafu};
use url::Url;

mod cache;
pub use cache::*;

type Result<T> = core::result::Result<T, HttpError>;

#[derive(Debug, Snafu)]
pub enum HttpError {
    #[snafu(display("failed to extract file name from URL"))]
    FileNameError,

    #[snafu(display("cache error"))]
    CacheError { source: CacheError },

    #[snafu(display("request error"))]
    RequestError { source: reqwest::Error },

    #[snafu(display("failed to deserialize content into YAML"))]
    YamlError { source: serde_yaml::Error },
}

pub struct HttpClient {
    pub(crate) client: reqwest::Client,
    pub(crate) cache: Cache,
}

impl HttpClient {
    /// Creates a new [`HttpClient`] with caching capabilities.
    pub fn new(cache_settings: CacheSettings) -> Self {
        let cache = Cache::new(cache_settings);
        let client = reqwest::Client::new();

        Self { client, cache }
    }

    /// Retrieves plain data from the provided `url`.
    pub async fn get_plain_data(&self, url: Url) -> Result<String> {
        let file_name = Self::extract_file_name(&url)?;

        match self
            .cache
            .retrieve(&file_name)
            .await
            .context(CacheSnafu {})?
        {
            CacheStatus::Hit(content) => Ok(content),
            CacheStatus::Expired | CacheStatus::Miss => {
                let content = self.get(url).await?;
                self.cache
                    .store(&file_name, &content)
                    .await
                    .context(CacheSnafu {})?;

                Ok(content)
            }
        }
    }

    /// Retrieves data from the provided `url` and tries to deserialize it
    /// into YAML.
    pub async fn get_yaml_data<T>(&self, url: Url) -> Result<T>
    where
        T: for<'a> Deserialize<'a> + Sized,
    {
        let file_name = Self::extract_file_name(&url)?;

        let content = match self
            .cache
            .retrieve(&file_name)
            .await
            .context(CacheSnafu {})?
        {
            CacheStatus::Hit(content) => content,
            CacheStatus::Expired | CacheStatus::Miss => {
                let content = self.get(url).await?;
                self.cache
                    .store(&file_name, &content)
                    .await
                    .context(CacheSnafu {})?;

                content
            }
        };

        serde_yaml::from_str(&content).context(YamlSnafu {})
    }

    /// Internal call which executes a HTTP GET request to `url`.
    async fn get(&self, url: Url) -> Result<String> {
        let req = self.client.get(url).build().context(RequestSnafu {})?;
        let result = self.client.execute(req).await.context(RequestSnafu {})?;

        result.text().await.context(RequestSnafu {})
    }

    /// Returns the last URL path segment as the filename. It ensures that the
    /// last segment contains at least one dot.
    fn extract_file_name(url: &Url) -> Result<String> {
        let segment = url
            .path_segments()
            .ok_or(HttpError::FileNameError)?
            .last()
            .ok_or(HttpError::FileNameError)?;

        ensure!(segment.contains('.'), FileNameSnafu {});
        Ok(segment.to_string())
    }
}
