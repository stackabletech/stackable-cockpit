use std::time::Duration;

use clap::{Args, Subcommand};
use comfy_table::{presets::UTF8_FULL, ColumnConstraint, Table, Width};
use snafu::{ResultExt, Snafu};
use stackable::xfer::cache::{Cache, CacheError};

use crate::cli::{CacheSettingsError, Cli};

#[derive(Debug, Args)]
pub struct CacheArgs {
    #[command(subcommand)]
    subcommand: CacheCommands,
}

#[derive(Debug, Subcommand)]
pub enum CacheCommands {
    /// List cached files
    #[command(aliases(["ls"]))]
    List,

    /// Clean cached files
    #[command(aliases(["rm", "purge"]))]
    Clean,
}

#[derive(Debug, Snafu)]
pub enum CacheCmdError {
    #[snafu(display("cache settings error"))]
    CacheSettingsError { source: CacheSettingsError },

    #[snafu(display("cache error"))]
    CacheError { source: CacheError },
}

impl CacheArgs {
    pub async fn run(&self, common_args: &Cli) -> Result<String, CacheCmdError> {
        match self.subcommand {
            CacheCommands::List => list_cmd(common_args).await,
            CacheCommands::Clean => clean_cmd(common_args).await,
        }
    }
}

async fn list_cmd(common_args: &Cli) -> Result<String, CacheCmdError> {
    let cache = Cache::new(common_args.cache_settings().context(CacheSettingsSnafu)?);
    cache.init().await.context(CacheSnafu)?;

    let files = cache.list().await.context(CacheSnafu)?;

    if files.is_empty() {
        return Ok("No cached files".into());
    }

    let mut table = Table::new();
    table
        .set_header(vec!["FILE", "LAST SYNC"])
        .set_constraints(vec![ColumnConstraint::UpperBoundary(Width::Percentage(80))])
        .load_preset(UTF8_FULL);

    for (path, modified) in files {
        let file_path = path.to_str().unwrap_or("Invalid UTF-8 Path").to_string();
        let modified = modified
            .elapsed()
            .unwrap_or(Duration::ZERO)
            .as_secs()
            .to_string();

        table.add_row(vec![file_path, format!("{modified} seconds ago")]);
    }

    Ok(table.to_string())
}

async fn clean_cmd(common_args: &Cli) -> Result<String, CacheCmdError> {
    let cache = Cache::new(common_args.cache_settings().context(CacheSettingsSnafu)?);
    cache.init().await.context(CacheSnafu)?;
    cache.purge().await.context(CacheSnafu)?;

    Ok("Cleaned cached files".into())
}
