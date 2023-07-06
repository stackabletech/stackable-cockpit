use std::{fs, io, time::Duration};

use clap::{Args, Subcommand};
use comfy_table::{presets::UTF8_FULL, Table};
use snafu::{ResultExt, Snafu};
use xdg::BaseDirectoriesError;

use crate::constants::CACHE_HOME_PATH;

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
    #[snafu(display("io error: {source}"))]
    IoError { source: io::Error },

    #[snafu(display("xdg error: {source}"))]
    XdgError { source: BaseDirectoriesError },
}

impl CacheArgs {
    pub fn run(&self) -> Result<String, CacheCmdError> {
        match self.subcommand {
            CacheCommands::List => list_cmd(),
            CacheCommands::Clean => clean_cmd(),
        }
    }
}

fn list_cmd() -> Result<String, CacheCmdError> {
    let cache_dir = xdg::BaseDirectories::with_prefix(CACHE_HOME_PATH)
        .context(XdgSnafu)?
        .get_cache_home();

    fs::create_dir_all(cache_dir.clone()).context(IoSnafu)?;

    let mut files = fs::read_dir(cache_dir)
        .context(IoSnafu)?
        .map(|res| {
            let entry = res?;
            Ok((entry.path(), entry.metadata()?.modified()?))
        })
        .collect::<Result<Vec<_>, io::Error>>()
        .context(IoSnafu)?;

    files.sort();

    if files.is_empty() {
        return Ok("No cached files".into());
    }

    let mut table = Table::new();
    table
        .set_header(vec!["FILE", "LAST SYNC"])
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

fn clean_cmd() -> Result<String, CacheCmdError> {
    let cache_dir = xdg::BaseDirectories::with_prefix(CACHE_HOME_PATH)
        .context(XdgSnafu)?
        .get_cache_home();

    fs::remove_dir_all(cache_dir.clone()).context(IoSnafu)?;
    fs::create_dir_all(cache_dir).context(IoSnafu)?;

    Ok("Cleaned cached files".into())
}
