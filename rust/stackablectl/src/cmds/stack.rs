use clap::{Args, Subcommand};
use comfy_table::{
    ContentArrangement, Table,
    presets::{NOTHING, UTF8_FULL},
};
use snafu::{OptionExt as _, ResultExt, Snafu, ensure};
use stackable_cockpit::{
    common::list,
    constants::{DEFAULT_NAMESPACE, DEFAULT_OPERATOR_NAMESPACE},
    platform::{
        operator::ChartSourceType,
        release,
        stack::{self, StackInstallParameters},
    },
    utils::{
        k8s::{self, Client},
        path::PathOrUrlParseError,
    },
    xfer::{self, cache::Cache},
};
use stackable_operator::kvp::{LabelError, Labels};
use tracing::{debug, info, instrument};

use crate::{
    args::{CommonClusterArgs, CommonClusterArgsError, CommonNamespaceArgs},
    cli::{Cli, OutputType},
};

#[derive(Debug, Args)]
pub struct StackArgs {
    #[command(subcommand)]
    subcommand: StackCommands,

    /// Target a specific Stackable release
    #[arg(long, global = true)]
    release: Option<String>,
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

    #[snafu(display("no release '{release}'"))]
    NoSuchRelease { release: String },

    #[snafu(display("failed to get latest release"))]
    LatestRelease,

    #[snafu(display("failed to build stack/release list"))]
    BuildList { source: list::Error },

    #[snafu(display("failed to install local cluster"))]
    InstallCluster { source: CommonClusterArgsError },

    #[snafu(display("failed to install stack {stack_name:?}"))]
    InstallStack {
        source: stack::Error,
        stack_name: String,
    },

    #[snafu(display("failed to build labels for stack resources"))]
    BuildLabels { source: LabelError },

    #[snafu(display("failed to create Kubernetes client"))]
    KubeClientCreate { source: k8s::Error },
}

impl StackArgs {
    pub async fn run(&self, cli: &Cli, cache: Cache) -> Result<String, CmdError> {
        debug!("Handle stack args");

        let transfer_client = xfer::Client::new_with(cache);

        let release_files = cli.get_release_files().context(PathOrUrlParseSnafu)?;
        let release_list = release::ReleaseList::build(&release_files, &transfer_client)
            .await
            .context(BuildListSnafu)?;

        let release_branch = match &self.release {
            Some(release) => {
                ensure!(release_list.contains_key(release), NoSuchReleaseSnafu {
                    release
                });

                if release == "dev" {
                    "main".to_string()
                } else {
                    format!("release-{release}")
                }
            }
            None => {
                let (release_name, _) = release_list.first().context(LatestReleaseSnafu)?;
                format!("release-{release}", release = release_name,)
            }
        };

        let files = cli
            .get_stack_files(&release_branch)
            .context(PathOrUrlParseSnafu)?;
        let stack_list = stack::StackList::build(&files, &transfer_client)
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

#[instrument(skip_all)]
fn list_cmd(
    args: &StackListArgs,
    cli: &Cli,
    stack_list: stack::StackList,
) -> Result<String, CmdError> {
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

            for (index, (stack_name, stack)) in stack_list.iter().enumerate() {
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

#[instrument(skip_all)]
fn describe_cmd(
    args: &StackDescribeArgs,
    cli: &Cli,
    stack_list: stack::StackList,
) -> Result<String, CmdError> {
    info!(stack_name = %args.stack_name, "Describing stack");

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
    stack_list: stack::StackList,
    transfer_client: &xfer::Client,
) -> Result<String, CmdError> {
    info!(stack_name = %args.stack_name, "Installing stack");

    let files = cli.get_release_files().context(PathOrUrlParseSnafu)?;
    let release_list = release::ReleaseList::build(&files, transfer_client)
        .await
        .context(BuildListSnafu)?;

    match stack_list.get(&args.stack_name) {
        Some(stack_spec) => {
            let mut output = cli.result();

            // Install local cluster if needed
            args.local_cluster
                .install_if_needed()
                .await
                .context(InstallClusterSnafu)?;

            let client = Client::new().await.context(KubeClientCreateSnafu)?;

            // Construct labels which get attached to all dynamic objects which
            // are part of the stack.
            let labels = Labels::try_from([
                ("stackable.tech/managed-by", "stackablectl"),
                ("stackable.tech/stack", &args.stack_name),
                ("stackable.tech/vendor", "Stackable"),
            ])
            .context(BuildLabelsSnafu)?;

            let install_parameters = StackInstallParameters {
                operator_namespace: args.namespaces.operator_namespace.clone(),
                stack_namespace: args.namespaces.namespace.clone(),
                stack_name: args.stack_name.clone(),
                parameters: args.parameters.clone(),
                skip_release: args.skip_release,
                demo_name: None,
                labels,
                chart_source: ChartSourceType::from(cli.chart_type()),
            };

            stack_spec
                .install(release_list, install_parameters, &client, transfer_client)
                .await
                .context(InstallStackSnafu {
                    stack_name: args.stack_name.clone(),
                })?;

            let operator_cmd = format!(
                "stackablectl operator installed{}",
                if args.namespaces.operator_namespace != DEFAULT_OPERATOR_NAMESPACE {
                    format!(
                        " --operator-namespace {}",
                        args.namespaces.operator_namespace
                    )
                } else {
                    "".into()
                }
            );

            let stacklet_cmd = format!(
                "stackablectl stacklet list{}",
                if args.namespaces.namespace != DEFAULT_NAMESPACE {
                    format!(" --namespace {}", args.namespaces.namespace)
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
