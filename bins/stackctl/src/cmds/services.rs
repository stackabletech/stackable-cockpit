use clap::{Args, Subcommand};
use thiserror::Error;

use crate::cli::OutputType;

#[derive(Debug, Args)]
pub struct ServicesArgs {
    #[command(subcommand)]
    subcommand: ServiceCommands,
}

#[derive(Debug, Subcommand)]
pub enum ServiceCommands {
    /// List deployed services
    #[command(alias("ls"))]
    List(ServiceListArgs),
}

#[derive(Debug, Args)]
pub struct ServiceListArgs {
    /// Will display services of all namespaces, not only the current one
    #[arg(short, long)]
    all_namespaces: bool,

    /// Display credentials and secrets in the output
    #[arg(short, long)]
    show_credentials: bool,

    /// Display product versions in the output
    #[arg(long)]
    show_versions: bool,

    #[arg(short, long = "output", value_enum, default_value_t = Default::default())]
    output_type: OutputType,
}

#[derive(Debug, Error)]
pub enum ServicesError {}

impl ServicesArgs {
    pub fn run(&self) -> Result<String, ServicesError> {
        match &self.subcommand {
            ServiceCommands::List(args) => list_cmd(args),
        }
    }
}

fn list_cmd(args: &ServiceListArgs) -> Result<String, ServicesError> {
    todo!()
}
