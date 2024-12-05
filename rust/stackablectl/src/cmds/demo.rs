use clap::{Args, Subcommand};
use comfy_table::{
    presets::{NOTHING, UTF8_FULL},
    ContentArrangement, Row, Table,
};
use snafu::{OptionExt, ResultExt, Snafu};
use stackable_operator::kvp::{LabelError, Labels};
use tracing::{debug, info, instrument};

use stackable_cockpit::{
    common::list,
    constants::{DEFAULT_OPERATOR_NAMESPACE, DEFAULT_PRODUCT_NAMESPACE},
    platform::{
        demo::{self, DemoInstallParameters},
        release, stack,
    },
    utils::{
        k8s::{self, Client},
        path::PathOrUrlParseError,
    },
    xfer::{self, cache::Cache},
};

use crate::{
    args::{CommonClusterArgs, CommonClusterArgsError, CommonNamespaceArgs},
    cli::{Cli, OutputType},
};

#[derive(Debug, Args)]
pub struct DemoArgs {
    #[command(subcommand)]
    subcommand: DemoCommands,

    /// Target a specific Stackable release
    #[arg(long, global = true)]
    release: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum DemoCommands {
    /// List available demos
    #[command(alias("ls"))]
    List(DemoListArgs),

    /// Print out detailed demo information
    #[command(alias("desc"))]
    Describe(DemoDescribeArgs),

    /// Install a specific demo
    #[command(aliases(["i", "in"]))]
    Install(DemoInstallArgs),
}

#[derive(Debug, Args)]
pub struct DemoListArgs {
    #[arg(short, long = "output", value_enum, default_value_t = Default::default())]
    output_type: OutputType,
}

#[derive(Debug, Args)]
pub struct DemoDescribeArgs {
    /// Demo to describe
    #[arg(
        name = "DEMO",
        long_help = "Demo to describe

Use \"stackablectl demo list\" to display a list of available demos.
Use \"stackablectl demo install <DEMO>\" to install a specific demo."
    )]
    demo_name: String,

    #[arg(short, long = "output", value_enum, default_value_t = Default::default())]
    output_type: OutputType,
}

#[derive(Debug, Args)]
pub struct DemoInstallArgs {
    /// Demo to install
    #[arg(
        name = "DEMO",
        long_help = "Demo to install

Use \"stackablectl demo list\" to display a list of available demos.
Use \"stackablectl demo describe <DEMO>\" to display details about the specified demo."
    )]
    demo_name: String,

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

    /// List of parameters to use when installing the demo
    #[arg(long)]
    parameters: Vec<String>,

    #[command(flatten)]
    local_cluster: CommonClusterArgs,

    #[command(flatten)]
    namespaces: CommonNamespaceArgs,
}

#[derive(Debug, Args)]
pub struct DemoUninstallArgs {}

#[derive(Debug, Snafu)]
pub enum CmdError {
    #[snafu(display("failed to serialize YAML output"))]
    SerializeYamlOutput { source: serde_yaml::Error },

    #[snafu(display("failed to serialize JSON output"))]
    SerializeJsonOutput { source: serde_json::Error },

    #[snafu(display("no demo with name '{name}'"))]
    NoSuchDemo { name: String },

    #[snafu(display("no stack with name '{name}'"))]
    NoSuchStack { name: String },

    #[snafu(display("no release with name '{name}'"))]
    NoSuchRelease { name: String },

    #[snafu(display("empty release list"))]
    EmptyReleaseList,

    #[snafu(display("failed to build demo/stack/release list"))]
    BuildList { source: list::Error },

    #[snafu(display("path/url parse error"))]
    PathOrUrlParse { source: PathOrUrlParseError },

    #[snafu(display("failed to install local cluster"))]
    InstallCluster { source: CommonClusterArgsError },

    #[snafu(display("failed to install demo {demo_name:?}"))]
    InstallDemo {
        source: demo::Error,
        demo_name: String,
    },

    #[snafu(display("failed to build labels for demo resources"))]
    BuildLabels { source: LabelError },

    #[snafu(display("failed to create Kubernetes client"))]
    KubeClientCreate { source: k8s::Error },
}

impl DemoArgs {
    #[instrument]
    pub async fn run(&self, cli: &Cli, cache: Cache) -> Result<String, CmdError> {
        debug!("Handle demo args");

        let transfer_client = xfer::Client::new_with(cache);

        let release_files = cli.get_release_files().context(PathOrUrlParseSnafu)?;
        let release_list = release::ReleaseList::build(&release_files, &transfer_client)
            .await
            .context(BuildListSnafu)?;

        let release_branch = match &self.release {
            Some(release) => format!("release-{release}"),
            None => {
                let release = release_list
                    .inner()
                    .first()
                    .context(EmptyReleaseListSnafu)?
                    .0;

                format!("release-{release}")
            }
        };

        // Build demo list based on the (default) remote demo file, and additional files provided by the
        // STACKABLE_DEMO_FILES env variable or the --demo-files CLI argument.
        let demo_files = cli
            .get_demo_files(&release_branch)
            .context(PathOrUrlParseSnafu)?;

        let list = demo::List::build(&demo_files, &transfer_client)
            .await
            .context(BuildListSnafu)?;

        match &self.subcommand {
            DemoCommands::List(args) => list_cmd(args, cli, list).await,
            DemoCommands::Describe(args) => describe_cmd(args, cli, list).await,
            DemoCommands::Install(args) => install_cmd(args, cli, list, &transfer_client).await,
        }
    }
}

/// Print out a list of demos, either as a table (plain), JSON or YAML
#[instrument]
async fn list_cmd(args: &DemoListArgs, cli: &Cli, list: demo::List) -> Result<String, CmdError> {
    info!("Listing demos");

    match args.output_type {
        OutputType::Plain | OutputType::Table => {
            let (arrangement, preset) = match args.output_type {
                OutputType::Plain => (ContentArrangement::Disabled, NOTHING),
                _ => (ContentArrangement::Dynamic, UTF8_FULL),
            };

            let mut table = Table::new();
            table
                .set_header(vec!["#", "NAME", "STACK", "DESCRIPTION"])
                .set_content_arrangement(arrangement)
                .load_preset(preset);

            for (index, (demo_name, demo_spec)) in list.inner().iter().enumerate() {
                let row = Row::from(vec![
                    (index + 1).to_string(),
                    demo_name.clone(),
                    demo_spec.stack.clone(),
                    demo_spec.description.clone(),
                ]);
                table.add_row(row);
            }

            let mut result = cli.result();

            result
                .with_command_hint(
                    "stackablectl demo describe [OPTIONS] <DEMO>",
                    "display further information for the specified demo",
                )
                .with_command_hint(
                    "stackablectl demo install [OPTIONS] <DEMO>",
                    "install a demo",
                )
                .with_output(table.to_string());

            Ok(result.render())
        }
        OutputType::Json => serde_json::to_string(&list.inner()).context(SerializeJsonOutputSnafu),
        OutputType::Yaml => serde_yaml::to_string(&list.inner()).context(SerializeYamlOutputSnafu),
    }
}

/// Describe a specific demo by printing out a table (plain), JSON or YAML
#[instrument]
async fn describe_cmd(
    args: &DemoDescribeArgs,
    cli: &Cli,
    list: demo::List,
) -> Result<String, CmdError> {
    info!("Describing demo {}", args.demo_name);

    let demo = list.get(&args.demo_name).ok_or(CmdError::NoSuchDemo {
        name: args.demo_name.clone(),
    })?;

    match args.output_type {
        OutputType::Plain | OutputType::Table => {
            let arrangement = match args.output_type {
                OutputType::Plain => ContentArrangement::Disabled,
                _ => ContentArrangement::Dynamic,
            };

            let mut table = Table::new();
            table
                .set_content_arrangement(arrangement)
                .load_preset(NOTHING)
                .add_row(vec!["DEMO", &args.demo_name])
                .add_row(vec!["DESCRIPTION", &demo.description])
                .add_row_if(
                    |_, _| demo.documentation.is_some(),
                    vec!["DOCUMENTATION", demo.documentation.as_ref().unwrap()],
                )
                .add_row(vec!["STACK", &demo.stack])
                .add_row(vec!["LABELS", &demo.labels.join(", ")]);

            // TODO (Techassi): Add parameter output

            let mut result = cli.result();

            result
                .with_command_hint(
                    format!("stackablectl demo install {}", args.demo_name),
                    "install the demo",
                )
                .with_command_hint("stackablectl demo list", "list all available demos")
                .with_output(table.to_string());

            Ok(result.render())
        }
        OutputType::Json => serde_json::to_string(&demo).context(SerializeJsonOutputSnafu),
        OutputType::Yaml => serde_yaml::to_string(&demo).context(SerializeYamlOutputSnafu),
    }
}

/// Install a specific demo
#[instrument(skip(list, transfer_client))]
async fn install_cmd(
    args: &DemoInstallArgs,
    cli: &Cli,
    list: demo::List,
    transfer_client: &xfer::Client,
) -> Result<String, CmdError> {
    info!("Installing demo {}", args.demo_name);

    // Init result output and progress output
    let mut output = cli.result();

    let demo = list.get(&args.demo_name).ok_or(CmdError::NoSuchDemo {
        name: args.demo_name.clone(),
    })?;

    // TODO (Techassi): Try to move all this boilerplate code to build the lists out of here
    let files = cli.get_stack_files().context(PathOrUrlParseSnafu)?;
    let stack_list = stack::StackList::build(&files, transfer_client)
        .await
        .context(BuildListSnafu)?;

    let files = cli.get_release_files().context(PathOrUrlParseSnafu)?;
    let release_list = release::ReleaseList::build(&files, transfer_client)
        .await
        .context(BuildListSnafu)?;

    // Install local cluster if needed
    args.local_cluster
        .install_if_needed()
        .await
        .context(InstallClusterSnafu)?;

    let client = Client::new().await.context(KubeClientCreateSnafu)?;

    // Construct labels which get attached to all dynamic objects which
    // are part of the demo and stack.
    let labels = Labels::try_from([
        ("stackable.tech/managed-by", "stackablectl"),
        ("stackable.tech/demo", &args.demo_name),
        ("stackable.tech/vendor", "Stackable"),
    ])
    .context(BuildLabelsSnafu)?;

    let mut stack_labels = labels.clone();
    stack_labels
        .parse_insert(("stackable.tech/stack", &demo.stack))
        .context(BuildLabelsSnafu)?;

    let install_parameters = DemoInstallParameters {
        operator_namespace: args.namespaces.operator_namespace.clone(),
        product_namespace: args.namespaces.product_namespace.clone(),
        stack_parameters: args.stack_parameters.clone(),
        parameters: args.parameters.clone(),
        skip_release: args.skip_release,
        stack_labels,
        labels,
    };

    demo.install(
        stack_list,
        release_list,
        install_parameters,
        &client,
        transfer_client,
    )
    .await
    .context(InstallDemoSnafu {
        demo_name: args.demo_name.clone(),
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
        if args.namespaces.product_namespace != DEFAULT_PRODUCT_NAMESPACE {
            format!(" --product-namespace {}", args.namespaces.product_namespace)
        } else {
            "".into()
        }
    );

    output
        .with_command_hint(operator_cmd, "display the installed operators")
        .with_command_hint(stacklet_cmd, "display the installed stacklets")
        .with_output(format!("Installed demo '{}'", args.demo_name));

    Ok(output.render())
}
