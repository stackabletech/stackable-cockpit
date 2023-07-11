use clap::{Args, Subcommand};
use snafu::{ResultExt, Snafu};

use stackable::{
    common::ListError,
    platform::{
        demo::{DemoList, DemoSpecV2},
        release::ReleaseList,
        stack::{StackError, StackList},
    },
    utils::{params::IntoParametersError, path::PathOrUrlParseError},
};
use tracing::{debug, info, instrument};

use crate::{
    cli::{CacheSettingsError, Cli, CommonClusterArgs, CommonClusterArgsError, OutputType},
    output::{ResultOutput, TabledOutput},
};

#[derive(Debug, Args)]
pub struct DemoArgs {
    #[command(subcommand)]
    subcommand: DemoCommands,
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
    // #[command(aliases(["rm", "un"]))]
    // Uninstall(DemoUninstallArgs),
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

    /// List of parameters to use when installing the stack
    #[arg(short, long)]
    stack_parameters: Vec<String>,

    /// List of parameters to use when installing the demo
    #[arg(short, long)]
    parameters: Vec<String>,

    #[command(flatten)]
    local_cluster: CommonClusterArgs,
}

#[derive(Debug, Args)]
pub struct DemoUninstallArgs {}

#[derive(Debug, Snafu)]
pub enum DemoCmdError {
    #[snafu(display("io error"))]
    IoError { source: std::io::Error },

    #[snafu(display("unable to format yaml output"), context(false))]
    YamlOutputFormatError { source: serde_yaml::Error },

    #[snafu(display("unable to format json output"), context(false))]
    JsonOutputFormatError { source: serde_json::Error },

    #[snafu(display("no demo with name '{name}'"))]
    NoSuchDemo { name: String },

    #[snafu(display("no stack with name '{name}'"))]
    NoSuchStack { name: String },

    #[snafu(display("failed to convert input parameters to validated parameters: {source}"))]
    IntoParametersError { source: IntoParametersError },

    #[snafu(display("list error"))]
    ListError { source: ListError },

    #[snafu(display("stack error"))]
    StackError { source: StackError },

    #[snafu(display("path/url parse error"))]
    PathOrUrlParseError { source: PathOrUrlParseError },

    #[snafu(display("cache settings resolution error"), context(false))]
    CacheSettingsError { source: CacheSettingsError },

    #[snafu(display("cluster argument error"))]
    CommonClusterArgsError { source: CommonClusterArgsError },
}

impl ResultOutput for DemoList {
    const EMPTY_MESSAGE: &'static str = "No demos";
    type Error = DemoCmdError;
}

impl TabledOutput for DemoList {
    const COLUMNS: &'static [&'static str] = &["#", "NAME", "STACK", "DESCRIPTION"];
    type Row = Vec<String>;

    fn rows(&self) -> Vec<Self::Row> {
        self.inner()
            .iter()
            .enumerate()
            .map(|(index, (demo_name, demo_spec))| {
                vec![
                    (index + 1).to_string(),
                    demo_name.clone(),
                    demo_spec.stack.clone(),
                    demo_spec.description.clone(),
                ]
            })
            .collect()
    }
}

impl ResultOutput for DemoSpecV2 {
    type Error = DemoCmdError;
}

impl TabledOutput for DemoSpecV2 {
    const COLUMNS: &'static [&'static str] = &[];
    type Row = Vec<String>;

    fn rows(&self) -> Vec<Self::Row> {
        // TODO (Techassi): Add parameter output
        let mut rows = Vec::new();

        rows.push(vec!["DESCRIPTION".into(), self.description.clone()]);

        if let Some(doc) = &self.documentation {
            rows.push(vec!["DOCUMENTATION".into(), doc.clone()]);
        }

        rows.push(vec!["STACK".into(), self.stack.clone()]);
        rows.push(vec!["LABELS".into(), self.labels.join(", ")]);

        rows
    }
}

impl DemoArgs {
    #[instrument]
    pub async fn run(&self, common_args: &Cli) -> Result<String, DemoCmdError> {
        debug!("Handle demo args");

        // Build demo list based on the (default) remote demo file, and additional files provided by the
        // STACKABLE_DEMO_FILES env variable or the --demo-files CLI argument.
        let files = common_args.get_demo_files().context(PathOrUrlParseSnafu)?;

        let list = DemoList::build(&files, &common_args.cache_settings()?)
            .await
            .context(ListSnafu)?;

        match &self.subcommand {
            DemoCommands::List(args) => list_cmd(args, list).await,
            DemoCommands::Describe(args) => describe_cmd(args, list).await,
            DemoCommands::Install(args) => install_cmd(args, common_args, list).await,
            // DemoCommands::Uninstall(args) => uninstall_cmd(args, list),
        }
    }
}

/// Print out a list of demos, either as a table (plain), JSON or YAML
#[instrument]
async fn list_cmd(args: &DemoListArgs, list: DemoList) -> Result<String, DemoCmdError> {
    info!("Listing demos");

    Ok(list.output(args.output_type)?)
}

/// Describe a specific demo by printing out a table (plain), JSON or YAML
#[instrument]
async fn describe_cmd(args: &DemoDescribeArgs, list: DemoList) -> Result<String, DemoCmdError> {
    info!("Describing demo");

    let demo = list.get(&args.demo_name).ok_or(DemoCmdError::NoSuchDemo {
        name: args.demo_name.clone(),
    })?;

    Ok(demo.output(args.output_type)?)
}

/// Install a specific demo
#[instrument(skip(list))]
async fn install_cmd(
    args: &DemoInstallArgs,
    common_args: &Cli,
    list: DemoList,
) -> Result<String, DemoCmdError> {
    info!("Installing demo");

    // Get the demo spec by name from the list
    let demo_spec = list.get(&args.demo_name).ok_or(DemoCmdError::NoSuchDemo {
        name: args.demo_name.clone(),
    })?;

    args.local_cluster
        .install_if_needed(None)
        .await
        .context(CommonClusterArgsSnafu)?;

    // Build demo list based on the (default) remote demo file, and additional files provided by the
    // STACKABLE_DEMO_FILES env variable or the --demo-files CLI argument.
    let files = common_args.get_stack_files().context(PathOrUrlParseSnafu)?;

    let cache_settings = common_args.cache_settings()?;

    let stack_list = StackList::build(&files, &cache_settings)
        .await
        .context(ListSnafu)?;

    // Get the stack spec based on the name defined in the demo spec
    let stack_spec = stack_list
        .get(&demo_spec.stack)
        .ok_or(DemoCmdError::NoSuchStack {
            name: demo_spec.stack.clone(),
        })?;

    // TODO (Techassi): Try to move all this boilerplate code to build the lists out of here
    let files = common_args
        .get_release_files()
        .context(PathOrUrlParseSnafu)?;

    let release_list = ReleaseList::build(&files, &cache_settings)
        .await
        .context(ListSnafu)?;

    // Install local cluster if needed
    args.local_cluster
        .install_if_needed(None)
        .await
        .context(CommonClusterArgsSnafu)?;

    // Install the stack
    stack_spec
        .install(release_list, &common_args.operator_namespace)
        .context(StackSnafu)?;

    // Install stack manifests
    stack_spec
        .install_stack_manifests(&args.stack_parameters, &common_args.operator_namespace)
        .await
        .context(StackSnafu)?;

    // Install demo manifests
    stack_spec
        .install_demo_manifests(
            &demo_spec.manifests,
            &demo_spec.parameters,
            &args.parameters,
            &common_args.operator_namespace,
        )
        .await
        .context(StackSnafu)?;

    Ok("".into())
}

// fn uninstall_cmd(_args: &DemoUninstallArgs, _list: DemoList) -> Result<String, DemoCmdError> {
//     todo!()
// }
