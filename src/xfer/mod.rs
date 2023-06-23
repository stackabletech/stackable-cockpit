use std::collections::HashMap;

use serde::Deserialize;
use snafu::{ensure, ResultExt, Snafu};
use tera::{Context, Tera};
use url::Url;

mod cache;
pub use cache::*;

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

    /// Retrieves plain data from the provided `url`.
    pub async fn get_plain_data(&self, url: &Url) -> Result<String> {
        self.get_from_cache_or_remote(url).await
    }

    /// Retrieves plain data from the provided `url` and applies templating
    /// to it. Variables inside handlebars are replaced with values provided
    /// by the `parameters`, which is a key-value map.
    pub async fn get_templated_plain_data(
        &self,
        url: &Url,
        parameters: &HashMap<String, String>,
    ) -> Result<String> {
        let content = self.get_from_cache_or_remote(url).await?;
        Self::render_template(&content, parameters)
    }

    /// Retrieves data from the provided `url` and tries to deserialize it
    /// into YAML.
    pub async fn get_yaml_data<T>(&self, url: &Url) -> Result<T>
    where
        T: for<'a> Deserialize<'a> + Sized,
    {
        let content = self.get_from_cache_or_remote(url).await?;
        serde_yaml::from_str(&content).context(YamlSnafu {})
    }

    /// Retrieves data from the provided `url`,  applies templating and tries
    /// to deserialize it into YAML Variables inside handlebars are replaced
    /// with values provided by the `parameters`, which is a key-value map.
    pub async fn get_templated_yaml_data<T>(
        &self,
        url: &Url,
        parameters: &HashMap<String, String>,
    ) -> Result<T>
    where
        T: for<'a> Deserialize<'a> + Sized,
    {
        let content = self.get_from_cache_or_remote(url).await?;
        let content = Self::render_template(&content, parameters)?;
        serde_yaml::from_str(&content).context(YamlSnafu {})
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
                let content = self.get(url).await?;
                self.cache
                    .store(&file_name, &content)
                    .await
                    .context(CacheSnafu {})?;

                Ok(content)
            }
        }
    }

    /// Internal call which executes a HTTP GET request to `url`.
    async fn get(&self, url: &Url) -> Result<String> {
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

    /// Creates a template rendering context and inserts key-value pairs.
    fn create_templating_context(parameters: &HashMap<String, String>) -> Context {
        // Create templating context
        let mut context = Context::new();

        // Fill context with parameters
        for (name, value) in parameters {
            context.insert(name, value)
        }

        context
    }

    /// Renders templated content and returns it as a [`String`].
    fn render_template(content: &str, parameters: &HashMap<String, String>) -> Result<String> {
        let context = Self::create_templating_context(parameters);
        Tera::one_off(content, &context, true).context(TemplatingSnafu)
    }
}
