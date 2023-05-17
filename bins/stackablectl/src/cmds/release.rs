use clap::{Args, Subcommand};
use snafu::Snafu;
use stackable::constants::DEFAULT_LOCAL_CLUSTER_NAME;

use crate::cli::{ClusterType, OutputType};

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
    Describe(ReleaseDescribeArgs),

    /// Install a specific release
    #[command(aliases(["i", "in"]))]
    Install(ReleaseInstallArgs),

    /// Uninstall a release
    #[command(aliases(["rm", "un"]))]
    Uninstall(ReleaseUninstallArgs),
}

#[derive(Debug, Args)]
pub struct ReleaseListArgs {
    #[arg(short, long = "output", value_enum, default_value_t = Default::default())]
    output_type: OutputType,
}

#[derive(Debug, Args)]
pub struct ReleaseDescribeArgs {
    #[arg(short, long = "output", value_enum, default_value_t = Default::default())]
    output_type: OutputType,
}

#[derive(Debug, Args)]
pub struct ReleaseInstallArgs {
    /// Whitelist of product operators to install
    #[arg(short, long = "include", group = "products")]
    included_products: Vec<String>,

    /// Blacklist of product operators to install
    #[arg(short, long = "exclude", group = "products")]
    excluded_products: Vec<String>,

    /// Type of local cluster to use for testing
    #[arg(short, long, value_enum, value_name = "CLUSTER_TYPE", default_value_t = ClusterType::default())]
    #[arg(
        long_help = "If specified, a local Kubernetes cluster consisting of 4 nodes (1 for
control-plane and 3 workers) will be created for testing purposes. Currently
'kind' and 'minikube' are supported. Both require a working Docker
installation on the system."
    )]
    cluster: ClusterType,

    /// Name of the local cluster
    #[arg(long, default_value = DEFAULT_LOCAL_CLUSTER_NAME)]
    cluster_name: String,
}

#[derive(Debug, Args)]
pub struct ReleaseUninstallArgs {
    /// Name of the release to uninstall
    release_name: String,
}

#[derive(Debug, Snafu)]
pub enum ReleaseError {}

impl ReleaseArgs {
    pub fn run(&self) -> Result<String, ReleaseError> {
        match &self.subcommand {
            ReleaseCommands::List(args) => list_cmd(args),
            ReleaseCommands::Describe(args) => describe_cmd(args),
            ReleaseCommands::Install(args) => install_cmd(args),
            ReleaseCommands::Uninstall(args) => uninstall_cmd(args),
        }
    }
}

fn list_cmd(_args: &ReleaseListArgs) -> Result<String, ReleaseError> {
    todo!()
}

fn describe_cmd(_args: &ReleaseDescribeArgs) -> Result<String, ReleaseError> {
    todo!()
}

fn install_cmd(_args: &ReleaseInstallArgs) -> Result<String, ReleaseError> {
    todo!()
}

fn uninstall_cmd(_args: &ReleaseUninstallArgs) -> Result<String, ReleaseError> {
    todo!()
}
