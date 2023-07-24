use std::{
    fs, io,
    path::PathBuf,
    time::{Duration, SystemTime},
};

use clap::{Args, Subcommand};
use thiserror::Error;
use xdg::BaseDirectoriesError;

use crate::{
    cli::OutputType,
    constants::CACHE_HOME_PATH,
    output::{ResultOutput, TabledOutput},
};

#[derive(Debug, Args)]
pub struct CacheArgs {
    #[command(subcommand)]
    subcommand: CacheCommands,
}

#[derive(Debug, Subcommand)]
pub enum CacheCommands {
    /// List cached files
    #[command(aliases(["ls"]))]
    List(CacheListArgs),

    /// Clean cached files
    #[command(aliases(["rm", "purge"]))]
    Clean,
}

#[derive(Debug, Args)]
pub struct CacheListArgs {
    #[arg(short, long = "output", value_enum, default_value_t = Default::default())]
    output_type: OutputType,
}

#[derive(Debug, Error)]
pub enum CacheCmdError {
    #[error("io error")]
    IoError(#[from] io::Error),

    #[error("xdg error")]
    XdgError(#[from] BaseDirectoriesError),

    #[error("unable to format yaml output")]
    YamlOutputFormatError(#[from] serde_yaml::Error),

    #[error("unable to format json output")]
    JsonOutputFormatError(#[from] serde_json::Error),
}

impl ResultOutput for Vec<(PathBuf, SystemTime)> {
    const EMPTY_MESSAGE: &'static str = "No cached files";
    type Error = CacheCmdError;
}

impl TabledOutput for Vec<(PathBuf, SystemTime)> {
    const COLUMNS: &'static [&'static str] = &["FILE", "LAST SYNC"];
    type Row = Vec<String>;

    fn rows(&self) -> Vec<Self::Row> {
        self.iter()
            .map(|(path, modified)| {
                vec![
                    path.to_string_lossy().to_string(),
                    format!(
                        "{} seconds ago",
                        modified.elapsed().unwrap_or(Duration::ZERO).as_secs()
                    ),
                ]
            })
            .collect()
    }
}

impl CacheArgs {
    pub fn run(&self) -> Result<String, CacheCmdError> {
        match &self.subcommand {
            CacheCommands::List(args) => list_cmd(args),
            CacheCommands::Clean => clean_cmd(),
        }
    }
}

fn list_cmd(args: &CacheListArgs) -> Result<String, CacheCmdError> {
    let cache_dir = xdg::BaseDirectories::with_prefix(CACHE_HOME_PATH)?.get_cache_home();
    fs::create_dir_all(cache_dir.clone())?;

    let mut files = fs::read_dir(cache_dir)?
        .map(|res| {
            let entry = res?;
            Ok((entry.path(), entry.metadata()?.modified()?))
        })
        .collect::<Result<Vec<_>, io::Error>>()?;

    files.sort();

    Ok(files.output(args.output_type)?)
}

fn clean_cmd() -> Result<String, CacheCmdError> {
    let cache_dir = xdg::BaseDirectories::with_prefix(CACHE_HOME_PATH)?.get_cache_home();

    fs::remove_dir_all(cache_dir.clone())?;
    fs::create_dir_all(cache_dir)?;

    Ok("Cleaned cached files".into())
}
