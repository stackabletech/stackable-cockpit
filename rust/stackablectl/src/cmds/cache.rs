use std::time::Duration;

use clap::{Args, Subcommand};
use comfy_table::{presets::UTF8_FULL, ColumnConstraint, Table, Width};
use snafu::{ResultExt, Snafu};
use stackable_cockpit::xfer::cache::{self, Cache, DeleteFilter};
use tracing::{info, instrument};

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
    Clean(CacheCleanArgs),
}

#[derive(Debug, Args)]
pub struct CacheCleanArgs {
    /// Only remove outdated files in the cache
    #[arg(long = "old", visible_aliases(["outdated"]))]
    only_remove_old_files: bool,
}

#[derive(Debug, Snafu)]
pub enum CmdError {
    #[snafu(display("cache settings error"))]
    CacheSettingsError { source: CacheSettingsError },

    #[snafu(display("cache error"))]
    CacheError { source: cache::Error },
}

impl CacheArgs {
    pub async fn run(&self, cli: &Cli, cache: Cache) -> Result<String, CmdError> {
        match &self.subcommand {
            CacheCommands::List => list_cmd(cache, cli).await,
            CacheCommands::Clean(args) => clean_cmd(args, cache).await,
        }
    }
}

#[instrument(skip_all)]
async fn list_cmd(cache: Cache, cli: &Cli) -> Result<String, CmdError> {
    info!("Listing cached files");

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

    let mut output = cli.output();
    output
        .add_command_hint("stackablectl cache clean", "to clean all cached files")
        .set_output(table.to_string());

    Ok(output.render())
}

#[instrument(skip_all)]
async fn clean_cmd(args: &CacheCleanArgs, cache: Cache) -> Result<String, CmdError> {
    info!("Cleaning cached files");

    let delete_filter = if args.only_remove_old_files {
        DeleteFilter::OnlyExpired
    } else {
        DeleteFilter::All
    };

    cache.purge(delete_filter).await.context(CacheSnafu)?;
    Ok("Cleaned cached files".into())
}
