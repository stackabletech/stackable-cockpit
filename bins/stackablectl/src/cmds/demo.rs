use clap::{Args, Subcommand};
use comfy_table::{
    presets::{NOTHING, UTF8_FULL},
    ContentArrangement, Row, Table,
};
use stackable::{
    common::ListError,
    constants::DEFAULT_LOCAL_CLUSTER_NAME,
    platform::{
        demo::DemoList,
        stack::{StackError, StackList},
    },
    utils::{
        params::IntoParametersError,
        path::PathOrUrlParseError,
        read::{CacheSettings, ReadError},
    },
};
use thiserror::Error;
use xdg::BaseDirectoriesError;

use crate::{
    cli::{Cli, ClusterType, OutputType},
    constants::{
        CACHE_DEMOS_PATH, CACHE_HOME_PATH, CACHE_STACKS_PATH, REMOTE_DEMO_FILE, REMOTE_STACK_FILE,
    },
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

    #[command(aliases(["rm", "un"]))]
    Uninstall(DemoUninstallArgs),
}

#[derive(Debug, Args)]
pub struct DemoListArgs {
    #[arg(short, long = "output", value_enum, default_value_t = Default::default())]
    output_type: OutputType,
}

#[derive(Debug, Args)]
pub struct DemoDescribeArgs {
    /// Demo to describe
    #[arg(name = "DEMO")]
    demo_name: String,

    #[arg(short, long = "output", value_enum, default_value_t = Default::default())]
    output_type: OutputType,
}

#[derive(Debug, Args)]
pub struct DemoInstallArgs {
    /// Demo to install
    #[arg(name = "DEMO")]
    demo_name: String,

    /// List of parameters to use when installing the stack
    #[arg(short, long)]
    stack_parameters: Vec<String>,

    /// List of parameters to use when installing the demo
    #[arg(short, long)]
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

#[derive(Debug, Args)]
pub struct DemoUninstallArgs {}

#[derive(Debug, Error)]
pub enum DemoCmdError {
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("read error: {0}")]
    ReadError(#[from] ReadError),

    #[error("yaml error: {0}")]
    YamlError(#[from] serde_yaml::Error),

    #[error("json error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("no demo with name '{0}'")]
    NoSuchDemo(String),

    #[error("no stack with name '{0}'")]
    NoSuchStack(String),

    #[error("failed to convert input parameters to validated parameters: {0}")]
    IntoParametersError(#[from] IntoParametersError),

    #[error("list error: {0}")]
    ListError(#[from] ListError),

    #[error("stack error: {0}")]
    StackError(#[from] StackError),

    #[error("path/url parse error: {0}")]
    PathOrUrlParseError(#[from] PathOrUrlParseError),

    #[error("xdg base directory error: {0}")]
    XdgError(#[from] BaseDirectoriesError),
}

impl DemoArgs {
    pub async fn run(&self, common_args: &Cli) -> Result<String, DemoCmdError> {
        // Build demo list based on the (default) remote demo file, and additional files provided by the
        // STACKABLE_DEMO_FILES env variable or the --demo-files CLI argument.
        let files = common_args.get_demo_files()?;
        let cache_file_path = xdg::BaseDirectories::with_prefix(CACHE_HOME_PATH)?
            .place_cache_file(CACHE_DEMOS_PATH)?;

        let cache_settings = CacheSettings::from((cache_file_path, !common_args.no_cache));
        let list = DemoList::build(REMOTE_DEMO_FILE, files, cache_settings).await?;

        match &self.subcommand {
            DemoCommands::List(args) => list_cmd(args, list).await,
            DemoCommands::Describe(args) => describe_cmd(args, list).await,
            DemoCommands::Install(args) => install_cmd(args, common_args, list).await,
            DemoCommands::Uninstall(args) => uninstall_cmd(args, list),
        }
    }
}

/// Print out a list of demos, either as a table (plain), JSON or YAML
async fn list_cmd(args: &DemoListArgs, list: DemoList) -> Result<String, DemoCmdError> {
    match args.output_type {
        OutputType::Plain => {
            let mut table = Table::new();

            table
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_header(vec!["NAME", "STACK", "DESCRIPTION"])
                .load_preset(UTF8_FULL);

            for (demo_name, demo_spec) in list.inner() {
                let row = Row::from(vec![
                    demo_name.clone(),
                    demo_spec.stack.clone(),
                    demo_spec.description.clone(),
                ]);
                table.add_row(row);
            }

            Ok(table.to_string())
        }
        OutputType::Json => Ok(serde_json::to_string(&list.inner())?),
        OutputType::Yaml => Ok(serde_yaml::to_string(&list.inner())?),
    }
}

/// Describe a specific demo by printing out a table (plain), JSON or YAML
async fn describe_cmd(args: &DemoDescribeArgs, list: DemoList) -> Result<String, DemoCmdError> {
    let demo = list
        .get(&args.demo_name)
        .ok_or(DemoCmdError::NoSuchDemo(args.demo_name.clone()))?;

    match args.output_type {
        OutputType::Plain => {
            let mut table = Table::new();
            table
                .load_preset(NOTHING)
                .set_content_arrangement(ContentArrangement::Dynamic)
                .add_row(vec!["DEMO", &args.demo_name])
                .add_row(vec!["DESCRIPTION", &demo.description]);

            if let Some(documentation) = &demo.documentation {
                table.add_row(vec!["DOCUMENTATION", documentation]);
            }

            table
                .add_row(vec!["STACK", &demo.stack])
                .add_row(vec!["LABELS", &demo.labels.join(", ")]);

            // TODO (Techassi): Add parameter output

            Ok(table.to_string())
        }
        OutputType::Json => Ok(serde_json::to_string(&demo)?),
        OutputType::Yaml => Ok(serde_yaml::to_string(&demo)?),
    }
}

/// Install a specific demo
async fn install_cmd(
    args: &DemoInstallArgs,
    common_args: &Cli,
    list: DemoList,
) -> Result<String, DemoCmdError> {
    // Get the demo spec by name from the list
    let demo_spec = list
        .get(&args.demo_name)
        .ok_or(DemoCmdError::NoSuchDemo(args.demo_name.clone()))?;

    // Build demo list based on the (default) remote demo file, and additional files provided by the
    // STACKABLE_DEMO_FILES env variable or the --demo-files CLI argument.
    let files = common_args.get_stack_files()?;

    let cache_file_path =
        xdg::BaseDirectories::with_prefix(CACHE_HOME_PATH)?.place_cache_file(CACHE_STACKS_PATH)?;

    let stack_list = StackList::build(
        REMOTE_STACK_FILE,
        files,
        (cache_file_path, !common_args.no_cache).into(),
    )
    .await?;

    // Get the stack spec based on the name defined in the demo spec
    let stack_spec = stack_list
        .get(&demo_spec.stack)
        .ok_or(DemoCmdError::NoSuchStack(demo_spec.stack.clone()))?;

    // let release_spec =

    // Install the stack
    stack_spec.install()?;

    // Install stack manifests
    stack_spec.install_stack_manifests(&args.stack_parameters)?;

    // Install demo manifests
    stack_spec.install_demo_manifests(&demo_spec.parameters, &args.parameters)?;

    Ok("".into())
}

fn uninstall_cmd(_args: &DemoUninstallArgs, _list: DemoList) -> Result<String, DemoCmdError> {
    todo!()
}
