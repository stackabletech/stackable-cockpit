use clap::{Args, Subcommand};
use comfy_table::{
    presets::{NOTHING, UTF8_FULL},
    ContentArrangement, Table,
};
use snafu::{ResultExt, Snafu};
use tracing::{debug, info, instrument};

use stackable_cockpit::{
    common::list,
    constants::{DEFAULT_OPERATOR_NAMESPACE, DEFAULT_PRODUCT_NAMESPACE},
    platform::{
        namespace, release,
        stack::{self, StackInstallParameters},
    },
    utils::path::PathOrUrlParseError,
    xfer::{cache::Cache, Client},
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
    PathOrUrlParse { source: PathOrUrlParseError },

    #[snafu(display("failed to serialize YAML output"))]
    SerializeYamlOutput { source: serde_yaml::Error },

    #[snafu(display("failed to serialize JSON output"))]
    SerializeJsonOutput { source: serde_json::Error },

    #[snafu(display("failed to install stack"))]
    StackInstall { source: stack::Error },

    #[snafu(display("failed to build stack/release list"))]
    BuildList { source: list::Error },

    #[snafu(display("cluster argument error"))]
    CommonClusterArgs { source: CommonClusterArgsError },

    #[snafu(display("failed to create namespace '{namespace}'"))]
    NamespaceCreate {
        source: namespace::Error,
        namespace: String,
    },
}

impl StackArgs {
    pub async fn run(&self, cli: &Cli, cache: Cache) -> Result<String, CmdError> {
        debug!("Handle stack args");

        let transfer_client = Client::new_with(cache);
        let files = cli.get_stack_files().context(PathOrUrlParseSnafu)?;
        let stack_list = stack::List::build(&files, &transfer_client)
            .await
            .context(BuildListSnafu)?;

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
fn list_cmd(args: &StackListArgs, cli: &Cli, stack_list: stack::List) -> Result<String, CmdError> {
    info!("Listing stacks");

    match args.output_type {
        OutputType::Plain | OutputType::Table => {
            let (arrangement, preset) = match args.output_type {
                OutputType::Plain => (ContentArrangement::Disabled, NOTHING),
                _ => (ContentArrangement::Dynamic, UTF8_FULL),
            };

            let mut table = Table::new();
            table
                .set_header(vec!["#", "STACK", "RELEASE", "DESCRIPTION"])
                .set_content_arrangement(arrangement)
                .load_preset(preset);

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

            Ok(result.render())
        }
        OutputType::Json => serde_json::to_string(&stack_list).context(SerializeJsonOutputSnafu),
        OutputType::Yaml => serde_yaml::to_string(&stack_list).context(SerializeYamlOutputSnafu),
    }
}

#[instrument]
fn describe_cmd(
    args: &StackDescribeArgs,
    cli: &Cli,
    stack_list: stack::List,
) -> Result<String, CmdError> {
    info!("Describing stack {}", args.stack_name);

    match stack_list.get(&args.stack_name) {
        Some(stack) => match args.output_type {
            OutputType::Plain | OutputType::Table => {
                let arrangement = match args.output_type {
                    OutputType::Plain => ContentArrangement::Disabled,
                    _ => ContentArrangement::Dynamic,
                };

                let mut table = Table::new();

                let mut parameter_table = Table::new();

                parameter_table
                    .set_header(vec!["NAME", "DESCRIPTION", "DEFAULT VALUE"])
                    .set_content_arrangement(arrangement.clone())
                    .load_preset(NOTHING);

                for parameter in &stack.parameters {
                    parameter_table.add_row(vec![
                        parameter.name.clone(),
                        parameter.description.clone(),
                        parameter.default.clone(),
                    ]);
                }

                table
                    .set_content_arrangement(arrangement)
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

                Ok(result.render())
            }
            OutputType::Json => serde_json::to_string(&stack).context(SerializeJsonOutputSnafu),
            OutputType::Yaml => serde_yaml::to_string(&stack).context(SerializeYamlOutputSnafu),
        },
        None => Ok("No such stack".into()),
    }
}

#[instrument(skip(cli, stack_list, transfer_client))]
async fn install_cmd(
    args: &StackInstallArgs,
    cli: &Cli,
    stack_list: stack::List,
    transfer_client: &Client,
) -> Result<String, CmdError> {
    info!("Installing stack {}", args.stack_name);

    let files = cli.get_release_files().context(PathOrUrlParseSnafu)?;
    let release_list = release::List::build(&files, transfer_client)
        .await
        .context(BuildListSnafu)?;

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
            let mut output = cli.result();

            // Install local cluster if needed
            args.local_cluster
                .install_if_needed()
                .await
                .context(CommonClusterArgsSnafu)?;

            let install_parameters = StackInstallParameters {
                operator_namespace: operator_namespace.clone(),
                product_namespace: product_namespace.clone(),
                stack_name: args.stack_name.clone(),
                skip_release: args.skip_release,
                demo_name: None,
            };

            // TODO (Techassi): Add error variant, remove unused ones
            stack_spec
                .install(release_list, install_parameters, transfer_client)
                .await
                .unwrap();

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

            output
                .with_command_hint(operator_cmd, "display the installed operators")
                .with_command_hint(stacklet_cmd, "display the installed stacklets")
                .with_output(format!("Installed stack '{}'", args.stack_name));

            Ok(output.render())
        }
        None => Ok("No such stack".into()),
    }
}
