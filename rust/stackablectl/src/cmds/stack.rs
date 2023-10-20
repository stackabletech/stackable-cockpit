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
    platform::{
        namespace::{self, NamespaceError},
        release::ReleaseList,
        stack::{StackError, StackList},
    },
    utils::path::PathOrUrlParseError,
    xfer::{cache::Cache, FileTransferClient, FileTransferError},
};

use crate::{
    args::{CommonClusterArgs, CommonClusterArgsError, CommonNamespaceArgs},
    cli::{Cli, OutputType},
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
pub enum CmdError {
    #[snafu(display("path/url parse error"))]
    PathOrUrlParseError { source: PathOrUrlParseError },

    #[snafu(display("unable to format YAML output"))]
    YamlOutputFormatError { source: serde_yaml::Error },

    #[snafu(display("unable to format JSON output"))]
    JsonOutputFormatError { source: serde_json::Error },

    #[snafu(display("stack error"))]
    StackError { source: StackError },

    #[snafu(display("list error"))]
    ListError { source: ListError },

    #[snafu(display("cluster argument error"))]
    CommonClusterArgsError { source: CommonClusterArgsError },

    #[snafu(display("transfer error"))]
    TransferError { source: FileTransferError },

    #[snafu(display("failed to create namespace '{namespace}'"))]
    NamespaceError {
        source: NamespaceError,
        namespace: String,
    },
}

impl StackArgs {
    pub async fn run(&self, cli: &Cli, cache: Cache) -> Result<String, CmdError> {
        debug!("Handle stack args");

        let transfer_client = FileTransferClient::new_with(cache);
        let files = cli.get_stack_files().context(PathOrUrlParseSnafu)?;

        let stack_list = StackList::build(&files, &transfer_client)
            .await
            .context(ListSnafu)?;

        match &self.subcommand {
            StackCommands::List(args) => list_cmd(args, cli, stack_list),
            StackCommands::Describe(args) => describe_cmd(args, cli, stack_list),
            StackCommands::Install(args) => {
                install_cmd(args, cli, stack_list, &transfer_client).await
            }
        }
    }
}

#[instrument]
fn list_cmd(args: &StackListArgs, cli: &Cli, stack_list: StackList) -> Result<String, CmdError> {
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

            let mut result = cli.result();

            result
                .with_command_hint(
                    "stackablectl stack describe [OPTIONS] <STACK>",
                    "display further information for the specified stack",
                )
                .with_command_hint(
                    "stackablectl stack install [OPTIONS] <STACK>...",
                    "install a stack",
                )
                .with_output(table.to_string());

            // TODO (Techassi): Remove unwrap
            Ok(result.render().unwrap())
        }
        OutputType::Json => serde_json::to_string(&stack_list).context(JsonOutputFormatSnafu {}),
        OutputType::Yaml => serde_yaml::to_string(&stack_list).context(YamlOutputFormatSnafu {}),
    }
}

#[instrument]
fn describe_cmd(
    args: &StackDescribeArgs,
    cli: &Cli,
    stack_list: StackList,
) -> Result<String, CmdError> {
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

                let mut result = cli.result();

                result
                    .with_command_hint(
                        format!("stackablectl stack install {}", args.stack_name),
                        "install the stack",
                    )
                    .with_command_hint("stackablectl stack list", "list all available stacks")
                    .with_output(table.to_string());

                // TODO (Techassi): Remove unwrap
                Ok(result.render().unwrap())
            }
            OutputType::Json => serde_json::to_string(&stack).context(JsonOutputFormatSnafu {}),
            OutputType::Yaml => serde_yaml::to_string(&stack).context(YamlOutputFormatSnafu {}),
        },
        None => Ok("No such stack".into()),
    }
}

#[instrument(skip(cli, stack_list, transfer_client))]
async fn install_cmd(
    args: &StackInstallArgs,
    cli: &Cli,
    stack_list: StackList,
    transfer_client: &FileTransferClient,
) -> Result<String, CmdError> {
    info!("Installing stack {}", args.stack_name);

    let files = cli.get_release_files().context(PathOrUrlParseSnafu)?;

    let release_list = ReleaseList::build(&files, transfer_client)
        .await
        .context(ListSnafu)?;

    let product_namespace = args
        .namespaces
        .product_namespace
        .clone()
        .unwrap_or(DEFAULT_PRODUCT_NAMESPACE.into());

    let operator_namespace = args
        .namespaces
        .operator_namespace
        .clone()
        .unwrap_or(DEFAULT_OPERATOR_NAMESPACE.into());

    match stack_list.get(&args.stack_name) {
        Some(stack_spec) => {
            // Install local cluster if needed
            args.local_cluster
                .install_if_needed(None)
                .await
                .context(CommonClusterArgsSnafu)?;

            // Check perquisites
            stack_spec
                .check_prerequisites(&product_namespace)
                .await
                .context(StackSnafu)?;

            // Install release if not opted out
            if !args.skip_release {
                namespace::create_if_needed(operator_namespace.clone())
                    .await
                    .context(NamespaceSnafu {
                        namespace: operator_namespace.clone(),
                    })?;

                stack_spec
                    .install_release(release_list, &operator_namespace, &product_namespace)
                    .await
                    .context(StackSnafu)?;
            } else {
                info!("Skipping release installation during stack installation process");
            }

            // Create product namespace if needed
            namespace::create_if_needed(product_namespace.clone())
                .await
                .context(NamespaceSnafu {
                    namespace: product_namespace.clone(),
                })?;

            // Install stack
            stack_spec
                .install_stack_manifests(
                    &args.stack_parameters,
                    &product_namespace,
                    transfer_client,
                )
                .await
                .context(StackSnafu)?;

            let mut result = cli.result();

            let operator_cmd = format!(
                "stackablectl operator installed{}",
                if args.namespaces.operator_namespace.is_some() {
                    format!(" --operator-namespace {}", operator_namespace)
                } else {
                    "".into()
                }
            );

            let stacklet_cmd = format!(
                "stackablectl stacklet list{}",
                if args.namespaces.product_namespace.is_some() {
                    format!(" --product-namespace {}", product_namespace)
                } else {
                    "".into()
                }
            );

            result
                .with_command_hint(operator_cmd, "display the installed operators")
                .with_command_hint(stacklet_cmd, "display the installed stacklets")
                .with_output(format!("Installed stack '{}'", args.stack_name));

            // TODO (Techassi): Remove unwrap
            Ok(result.render().unwrap())
        }
        None => Ok("No such stack".into()),
    }
}
