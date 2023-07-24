use clap::{Args, Subcommand};
use thiserror::Error;
use tracing::{debug, info, instrument};

use stackable::{
    common::ListError,
    platform::{
        demo::{DemoList, DemoSpecV2},
        release::ReleaseList,
        stack::{StackError, StackList},
    },
    utils::{params::IntoParametersError, path::PathOrUrlParseError},
};

use crate::{
    cli::{CacheSettingsError, Cli, CommonClusterArgs, CommonClusterArgsError, OutputType},
    output::{ProgressOutput, ResultOutput, TabledOutput},
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

#[derive(Debug, Error)]
pub enum DemoCmdError {
    #[error("io error")]
    IoError(#[from] std::io::Error),

    #[error("unable to format yaml output")]
    YamlOutputFormatError(#[from] serde_yaml::Error),

    #[error("unable to format json output")]
    JsonOutputFormatError(#[from] serde_json::Error),

    #[error("no demo with name '{0}'")]
    NoSuchDemo(String),

    #[error("no stack with name '{0}'")]
    NoSuchStack(String),

    #[error("failed to convert input parameters to validated parameters")]
    IntoParametersError(#[from] IntoParametersError),

    #[error("list error")]
    ListError(#[from] ListError),

    #[error("stack error")]
    StackError(#[from] StackError),

    #[error("path/url parse error")]
    PathOrUrlParseError(#[from] PathOrUrlParseError),

    #[error("cache settings resolution error")]
    CacheSettingsError(#[from] CacheSettingsError),

    #[error("cluster argument error")]
    CommonClusterArgsError(#[from] CommonClusterArgsError),
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
        let files = common_args.get_demo_files()?;
        let list = DemoList::build(&files, &common_args.cache_settings()?).await?;

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
async fn list_cmd(args: &DemoListArgs, demo_list: DemoList) -> Result<String, DemoCmdError> {
    info!("Listing demos");

    Ok(demo_list.output(args.output_type)?)
}

/// Describe a specific demo by printing out a table (plain), JSON or YAML
#[instrument]
async fn describe_cmd(
    args: &DemoDescribeArgs,
    demo_list: DemoList,
) -> Result<String, DemoCmdError> {
    info!("Describing demo");

    let demo = demo_list
        .get(&args.demo_name)
        .ok_or(DemoCmdError::NoSuchDemo(args.demo_name.clone()))?;

    Ok(demo.output(args.output_type)?)
}

/// Install a specific demo
#[instrument(skip(demo_list))]
async fn install_cmd(
    args: &DemoInstallArgs,
    common_args: &Cli,
    demo_list: DemoList,
) -> Result<String, DemoCmdError> {
    info!("Installing demo");

    let mut pb = ProgressOutput::new();
    pb.add("Building stack list", Some(6));

    // Get the demo spec by name from the list
    let demo_spec = demo_list
        .get(&args.demo_name)
        .ok_or(DemoCmdError::NoSuchDemo(args.demo_name.clone()))?;

    // Build demo list based on the (default) remote demo file, and additional files provided by the
    // STACKABLE_DEMO_FILES env variable or the --demo-files CLI argument.
    let files = common_args.get_stack_files()?;
    let cache_settings = common_args.cache_settings()?;
    let stack_list = StackList::build(&files, &cache_settings).await?;

    // Get the stack spec based on the name defined in the demo spec
    let stack_spec = stack_list
        .get(&demo_spec.stack)
        .ok_or(DemoCmdError::NoSuchStack(demo_spec.stack.clone()))?;

    pb.tick_with_message("Building release list");

    // TODO (Techassi): Try to move all this boilerplate code to build the lists out of here
    let files = common_args.get_release_files()?;
    let release_list = ReleaseList::build(&files, &cache_settings).await?;

    pb.tick_with_message("Installing local cluster");

    // Install local cluster if needed
    args.local_cluster.install_if_needed(None).await?;

    pb.tick_with_message("Installing stack");

    // Install the stack
    stack_spec.install(release_list, &common_args.operator_namespace)?;

    pb.tick_with_message("Installing stack manifests");

    // Install stack manifests
    stack_spec
        .install_stack_manifests(&args.stack_parameters, &common_args.operator_namespace)
        .await?;

    pb.tick_with_message("Installing demo manifests");

    // Install demo manifests
    stack_spec
        .install_demo_manifests(
            &demo_spec.manifests,
            &demo_spec.parameters,
            &args.parameters,
            &common_args.operator_namespace,
        )
        .await?;

    Ok("".into())
}

// fn uninstall_cmd(_args: &DemoUninstallArgs, _list: DemoList) -> Result<String, DemoCmdError> {
//     todo!()
// }
