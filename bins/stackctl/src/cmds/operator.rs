use clap::{Args, Subcommand};
use stackable::{constants::DEFAULT_LOCAL_CLUSTER_NAME, types::platform::OperatorSpec};
use thiserror::Error;

use crate::cli::{ClusterType, OutputType};

#[derive(Debug, Args)]
pub struct OperatorArgs {
    #[command(subcommand)]
    subcommand: OperatorCommands,
}

#[derive(Debug, Subcommand)]
pub enum OperatorCommands {
    /// List available (or installed) operators
    #[command(alias("ls"))]
    List(OperatorListArgs),

    /// Print out detailed operator information
    #[command(alias("desc"))]
    Describe(OperatorDescribeArgs),

    /// Install one or more operators
    #[command(aliases(["i", "in"]))]
    Install(OperatorInstallArgs),

    /// Uninstall one or more operators
    #[command(aliases(["rm", "un"]))]
    Uninstall(OperatorUninstallArgs),

    /// List installed operator (same as list -i)
    Installed(OperatorInstalledArgs),
}

#[derive(Debug, Args)]
pub struct OperatorListArgs {
    /// List only installed operators
    #[arg(short = 'i', long = "installed")]
    list_installed: bool,

    #[arg(short, long = "output", value_enum, default_value_t = Default::default())]
    output_type: OutputType,
}

#[derive(Debug, Args)]
pub struct OperatorDescribeArgs {
    /// Operator to describe
    #[arg(name = "OPERATOR", required = true)]
    operator_name: String,

    #[arg(short, long = "output", value_enum, default_value_t = Default::default())]
    output_type: OutputType,
}

#[derive(Debug, Args)]
pub struct OperatorInstallArgs {
    /// Operator(s) to install
    #[arg(name = "OPERATORS", required = true)]
    #[arg(long_help = "Operator(s) to install

Must have the form 'name[=version]'. If no version is specified the latest
nightly version - build from the main branch - will be used. Possible valid
values are:

- superset
- superset=0.3.0
- superset=0.3.0-nightly
- superset=0.3.0-pr123

Use \"stackablectl operator list\" to list available versions for all operators
Use \"stackablectl operator describe <OPERATOR>\" to get available versions for one operator")]
    operators: Vec<OperatorSpec>,

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
pub struct OperatorUninstallArgs {
    /// One or more operators to uninstall
    #[arg(required = true)]
    operators: Vec<OperatorSpec>,
}

#[derive(Debug, Args)]
pub struct OperatorInstalledArgs {
    #[arg(short, long = "output", value_enum, default_value_t = Default::default())]
    output_type: OutputType,
}

#[derive(Debug, Error)]
pub enum OperatorError {}

impl OperatorArgs {
    pub fn run(&self) -> Result<String, OperatorError> {
        match &self.subcommand {
            OperatorCommands::List(args) => list_cmd(args),
            OperatorCommands::Describe(args) => describe_cmd(args),
            OperatorCommands::Install(args) => install_cmd(args),
            OperatorCommands::Uninstall(args) => uninstall_cmd(args),
            OperatorCommands::Installed(args) => installed_cmd(args),
        }
    }
}

fn list_cmd(args: &OperatorListArgs) -> Result<String, OperatorError> {
    if args.list_installed {
        return installed_cmd(&OperatorInstalledArgs {
            output_type: args.output_type.clone(),
        });
    }

    todo!()
}

fn describe_cmd(args: &OperatorDescribeArgs) -> Result<String, OperatorError> {
    todo!()
}

fn install_cmd(args: &OperatorInstallArgs) -> Result<String, OperatorError> {
    todo!()
}

fn uninstall_cmd(args: &OperatorUninstallArgs) -> Result<String, OperatorError> {
    todo!()
}

fn installed_cmd(args: &OperatorInstalledArgs) -> Result<String, OperatorError> {
    todo!()
}
