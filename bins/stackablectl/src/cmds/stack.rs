use clap::{Args, Subcommand};
use snafu::{ResultExt, Snafu};
use xdg::BaseDirectoriesError;

use stackable::{
    common::ListError,
    constants::DEFAULT_LOCAL_CLUSTER_NAME,
    platform::stack::StackList,
    utils::{path::PathOrUrlParseError, read::CacheSettings},
};

use crate::{
    cli::{Cli, ClusterType, OutputType},
    constants::CACHE_HOME_PATH,
};

#[derive(Debug, Args)]
pub struct StackArgs {
    #[command(subcommand)]
    subcommand: StackCommands,
}

#[derive(Debug, Subcommand)]
pub enum StackCommands {
    /// List available stacks
    #[command(alias("ls"))]
    List(StackListArgs),

    /// Describe a specific stack
    #[command(alias("desc"))]
    Describe(StackDescribeArgs),

    /// Install a specific stack
    #[command(aliases(["i", "in"]))]
    Install(StackInstallArgs),
}

#[derive(Debug, Args)]
pub struct StackListArgs {
    #[arg(short, long = "output", value_enum, default_value_t = Default::default())]
    output_type: OutputType,
}

#[derive(Debug, Args)]
pub struct StackDescribeArgs {
    /// Name of the stack to describe
    stack_name: String,

    #[arg(short, long = "output", value_enum, default_value_t = Default::default())]
    output_type: OutputType,
}

#[derive(Debug, Args)]
pub struct StackInstallArgs {
    /// List of parameters to use when installing the stack
    #[arg(short, long)]
    #[arg(long_help = "List of parameters to use when installing the stack

All parameters must have the format '<parameter>=<value>'. Multiple parameters
can be specified and are space separated. Valid parameters are:

- adminPassword=admin123
- adminUser=superuser
- 'endpoint=https://example.com port=1234'

Use \"stackablectl stack describe <STACK>\" to list available parameters for each stack.")]
    parameters: Vec<String>,

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

#[derive(Debug, Snafu)]
pub enum StackCmdError {
    #[snafu(display("path/url parse error"))]
    PathOrUrlParseError { source: PathOrUrlParseError },

    #[snafu(display("xdg base directory error"))]
    XdgError { source: BaseDirectoriesError },

    #[snafu(display("list error"))]
    ListError { source: ListError },
}

impl StackArgs {
    pub async fn run(&self, common_args: &Cli) -> Result<String, StackCmdError> {
        let files = common_args
            .get_stack_files()
            .context(PathOrUrlParseSnafu {})?;

        let cache_file_path = xdg::BaseDirectories::with_prefix(CACHE_HOME_PATH)
            .context(XdgSnafu {})?
            .get_cache_home();

        let cache_settings = CacheSettings::from((cache_file_path, !common_args.no_cache));
        let list = StackList::build(files, cache_settings)
            .await
            .context(ListSnafu {})?;

        match &self.subcommand {
            StackCommands::List(args) => list_cmd(args, list),
            StackCommands::Describe(args) => describe_cmd(args),
            StackCommands::Install(args) => install_cmd(args),
        }
    }
}

fn list_cmd(args: &StackListArgs, stacks: StackList) -> Result<String, StackCmdError> {
    todo!()
}

fn describe_cmd(_args: &StackDescribeArgs) -> Result<String, StackCmdError> {
    todo!()
}

fn install_cmd(_args: &StackInstallArgs) -> Result<String, StackCmdError> {
    todo!()
}
