use clap::{Args, Subcommand};
use comfy_table::{
    presets::{NOTHING, UTF8_FULL},
    ContentArrangement, Table,
};
use snafu::{ResultExt, Snafu};
use tracing::{debug, info, instrument};

use stackable_cockpit::{
    common::ListError,
    constants::{DEFAULT_OPERATOR_NAMESPACE, DEFAULT_PRODUCT_NAMESPACE},
    kube::KubeClientError,
    platform::{
        namespace,
        release::ReleaseList,
        stack::{StackError, StackList},
    },
    utils::path::PathOrUrlParseError,
    xfer::{FileTransferClient, FileTransferError},
};

use crate::{
    args::{CommonClusterArgs, CommonClusterArgsError, CommonNamespaceArgs},
    cli::{CacheSettingsError, Cli, OutputType},
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
    /// Name of the stack to describe
    stack_name: String,

    /// Skip the installation of the release during the stack install process
    #[arg(
        long,
        long_help = "Skip the installation of the release during the stack install process

Use \"stackablectl operator install [OPTIONS] <OPERATORS>...\" to install
required operators manually. Operators MUST be installed in the correct version.

Use \"stackablectl operator install --help\" to display more information on how
to specify operator versions."
    )]
    skip_release: bool,

    /// List of parameters to use when installing the stack
    #[arg(long)]
    stack_parameters: Vec<String>,

    /// List of parameters to use when installing the stack
    #[arg(long)]
    #[arg(long_help = "List of parameters to use when installing the stack

All parameters must have the format '<parameter>=<value>'. Multiple parameters
can be specified and are space separated. Valid parameters are:

- adminPassword=admin123
- adminUser=superuser
- 'endpoint=https://example.com port=1234'

Use \"stackablectl stack describe <STACK>\" to list available parameters for each stack.")]
    parameters: Vec<String>,

    #[command(flatten)]
    local_cluster: CommonClusterArgs,

    #[command(flatten)]
    namespaces: CommonNamespaceArgs,
}

#[derive(Debug, Snafu)]
pub enum StackCmdError {
    #[snafu(display("path/url parse error"))]
    PathOrUrlParseError { source: PathOrUrlParseError },

    #[snafu(display("unable to format yaml output"))]
    YamlOutputFormatError { source: serde_yaml::Error },

    #[snafu(display("unable to format json output"))]
    JsonOutputFormatError { source: serde_json::Error },

    #[snafu(display("stack error"))]
    StackError { source: StackError },

    #[snafu(display("list error"))]
    ListError { source: ListError },

    #[snafu(display("cache settings resolution error"), context(false))]
    CacheSettingsError { source: CacheSettingsError },

    #[snafu(display("cluster argument error"))]
    CommonClusterArgsError { source: CommonClusterArgsError },

    #[snafu(display("transfer error"))]
    TransferError { source: FileTransferError },

    #[snafu(display("kube client error"))]
    KubeClientError { source: KubeClientError },
}

impl StackArgs {
    pub async fn run(&self, common_args: &Cli) -> Result<String, StackCmdError> {
        debug!("Handle stack args");

        let transfer_client = FileTransferClient::new(common_args.cache_settings()?)
            .await
            .context(TransferSnafu)?;

        let files = common_args.get_stack_files().context(PathOrUrlParseSnafu)?;

        let stack_list = StackList::build(&files, &transfer_client)
            .await
            .context(ListSnafu)?;

        match &self.subcommand {
            StackCommands::List(args) => list_cmd(args, stack_list),
            StackCommands::Describe(args) => describe_cmd(args, stack_list),
            StackCommands::Install(args) => {
                install_cmd(args, common_args, stack_list, &transfer_client).await
            }
        }
    }
}

#[instrument]
fn list_cmd(args: &StackListArgs, stack_list: StackList) -> Result<String, StackCmdError> {
    info!("Listing stacks");

    match args.output_type {
        OutputType::Plain => {
            let mut table = Table::new();

            table
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_header(vec!["#", "STACK", "RELEASE", "DESCRIPTION"])
                .load_preset(UTF8_FULL);

            for (index, (stack_name, stack)) in stack_list.inner().iter().enumerate() {
                table.add_row(vec![
                    (index + 1).to_string(),
                    stack_name.clone(),
                    stack.release.clone(),
                    stack.description.clone(),
                ]);
            }

            Ok(table.to_string())
        }
        OutputType::Json => serde_json::to_string(&stack_list).context(JsonOutputFormatSnafu {}),
        OutputType::Yaml => serde_yaml::to_string(&stack_list).context(YamlOutputFormatSnafu {}),
    }
}

#[instrument]
fn describe_cmd(args: &StackDescribeArgs, stack_list: StackList) -> Result<String, StackCmdError> {
    info!("Describing stack {}", args.stack_name);

    match stack_list.get(&args.stack_name) {
        Some(stack) => match args.output_type {
            OutputType::Plain => {
                let mut table = Table::new();

                let mut parameter_table = Table::new();

                parameter_table
                    .set_header(vec!["NAME", "DESCRIPTION", "DEFAULT VALUE"])
                    .set_content_arrangement(ContentArrangement::Dynamic)
                    .load_preset(NOTHING);

                for parameter in &stack.parameters {
                    parameter_table.add_row(vec![
                        parameter.name.clone(),
                        parameter.description.clone(),
                        parameter.default.clone(),
                    ]);
                }

                table
                    .set_content_arrangement(ContentArrangement::Dynamic)
                    .load_preset(NOTHING)
                    .add_row(vec!["STACK", args.stack_name.as_str()])
                    .add_row(vec!["DESCRIPTION", stack.description.as_str()])
                    .add_row(vec!["RELEASE", stack.release.as_str()])
                    .add_row(vec!["OPERATORS", stack.operators.join(", ").as_str()])
                    .add_row(vec!["LABELS", stack.labels.join(", ").as_str()])
                    .add_row(vec!["PARAMETERS", parameter_table.to_string().as_str()]);

                Ok(table.to_string())
            }
            OutputType::Json => serde_json::to_string(&stack).context(JsonOutputFormatSnafu {}),
            OutputType::Yaml => serde_yaml::to_string(&stack).context(YamlOutputFormatSnafu {}),
        },
        None => Ok("No such stack".into()),
    }
}

#[instrument]
async fn install_cmd(
    args: &StackInstallArgs,
    common_args: &Cli,
    stack_list: StackList,
    transfer_client: &FileTransferClient,
) -> Result<String, StackCmdError> {
    info!("Installing stack {}", args.stack_name);

    let files = common_args
        .get_release_files()
        .context(PathOrUrlParseSnafu)?;

    let release_list = ReleaseList::build(&files, transfer_client)
        .await
        .context(ListSnafu)?;

    // Install local cluster if needed
    args.local_cluster
        .install_if_needed(None)
        .await
        .context(CommonClusterArgsSnafu)?;

    let operator_namespace = args
        .namespaces
        .operator_namespace
        .clone()
        .unwrap_or(DEFAULT_OPERATOR_NAMESPACE.into());

    namespace::create_if_needed(operator_namespace.clone())
        .await
        .context(KubeClientSnafu)?;

    let product_namespace = args
        .namespaces
        .product_namespace
        .clone()
        .unwrap_or(DEFAULT_PRODUCT_NAMESPACE.into());

    namespace::create_if_needed(product_namespace.clone())
        .await
        .context(KubeClientSnafu)?;

    match stack_list.get(&args.stack_name) {
        Some(stack_spec) => {
            // Install the stack
            stack_spec
                .install(release_list, &operator_namespace, args.skip_release)
                .context(StackSnafu)?;

            // Install stack manifests
            stack_spec
                .install_stack_manifests(
                    &args.stack_parameters,
                    &product_namespace,
                    transfer_client,
                )
                .await
                .context(StackSnafu)?;

            let output = format!(
                "Installed stack {}\n\n\
            Use \"stackablectl operator installed{}\" to display the installed operators\n\
            Use \"stackablectl stacklet list{}\" to display the installed stacklets",
                args.stack_name,
                if args.namespaces.operator_namespace.is_some() {
                    format!(" --operator-namespace {}", operator_namespace)
                } else {
                    "".into()
                },
                if args.namespaces.product_namespace.is_some() {
                    format!(" --product-namespace {}", product_namespace)
                } else {
                    "".into()
                }
            );

            Ok(output)
        }
        None => Ok("No such stack".into()),
    }
}
