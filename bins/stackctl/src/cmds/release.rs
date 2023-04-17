use clap::{Args, Subcommand};
use thiserror::Error;

use crate::cli::OutputType;

#[derive(Debug, Args)]
pub struct ReleaseArgs {
    #[command(subcommand)]
    subcommand: ReleaseCommands,
}

#[derive(Debug, Subcommand)]
pub enum ReleaseCommands {
    /// List available releases
    #[command(alias("ls"))]
    List(ReleaseListArgs),

    /// Print out detailed release information
    #[command(alias("desc"))]
    Describe,

    /// Install a specific release
    #[command(alias("i"), alias("in"))]
    Install,

    /// Uninstall a release
    #[command(alias("rm"), alias("un"))]
    Uninstall,
}

#[derive(Debug, Args)]
pub struct ReleaseListArgs {
    #[arg(short, long = "output", value_enum, default_value_t = Default::default())]
    output_type: OutputType,
}

#[derive(Debug, Error)]
pub enum ReleaseError {}

impl ReleaseArgs {
    pub(crate) fn run(&self) -> Result<String, ReleaseError> {
        match &self.subcommand {
            ReleaseCommands::List(args) => self.list_cmd(args),
            ReleaseCommands::Describe => todo!(),
            ReleaseCommands::Install => todo!(),
            ReleaseCommands::Uninstall => todo!(),
        }
    }

    fn list_cmd(&self, args: &ReleaseListArgs) -> Result<String, ReleaseError> {
        todo!()
    }
}
