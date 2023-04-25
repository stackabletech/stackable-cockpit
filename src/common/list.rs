use std::{fs, path::PathBuf};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use url::{ParseError, Url};

use crate::utils::{
    path::PathOrUrl,
    read::{read_cached_yaml_data, read_yaml_data, CacheStatus, ReadError},
};

#[async_trait(?Send)]
pub trait Listed<L>: Sized
where
    L: for<'a> Deserialize<'a> + Serialize,
{
    type Error: std::fmt::Display
        + std::error::Error
        + From<ParseError>
        + From<ReadError>
        + From<serde_yaml::Error>
        + From<std::io::Error>;

    async fn build<U, T>(
        remote_url: U,
        env_files: T,
        arg_files: T,
        cache_file_path: PathBuf,
        use_cache: bool,
    ) -> Result<Self, Self::Error>
    where
        U: AsRef<str>,
        T: AsRef<[PathOrUrl]>;

    async fn get_default_file(
        url: Url,
        cache_file_path: PathBuf,
        use_cache: bool,
    ) -> Result<L, Self::Error> {
        let specs = if use_cache {
            match read_cached_yaml_data::<L>(cache_file_path.clone())? {
                CacheStatus::Hit(demos) => demos,
                CacheStatus::Expired | CacheStatus::Miss => {
                    let demos = read_yaml_data::<L>(url).await?;
                    fs::write(cache_file_path, serde_yaml::to_string(&demos)?)?;
                    demos
                }
            }
        } else {
            read_yaml_data::<L>(url).await?
        };

        Ok(specs)
    }
}
