use std::{
    fs, io,
    path::PathBuf,
    time::{Duration, SystemTime},
};

use clap::{Args, Subcommand};
use snafu::{ResultExt, Snafu};
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

#[derive(Debug, Snafu)]
pub enum CacheCmdError {
    #[snafu(display("io error"))]
    IoError { source: io::Error },

    #[snafu(display("xdg error"))]
    XdgError { source: BaseDirectoriesError },

    #[snafu(display("unable to format yaml output"), context(false))]
    YamlOutputFormatError { source: serde_yaml::Error },

    #[snafu(display("unable to format json output"), context(false))]
    JsonOutputFormatError { source: serde_json::Error },
}

impl ResultOutput for Vec<(PathBuf, SystemTime)> {
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

    Ok(files.output(args.output_type)?)
}

fn clean_cmd() -> Result<String, CacheCmdError> {
    let cache_dir = xdg::BaseDirectories::with_prefix(CACHE_HOME_PATH)
        .context(XdgSnafu)?
        .get_cache_home();

    fs::remove_dir_all(cache_dir.clone()).context(IoSnafu)?;
    fs::create_dir_all(cache_dir).context(IoSnafu)?;

    Ok("Cleaned cached files".into())
}
