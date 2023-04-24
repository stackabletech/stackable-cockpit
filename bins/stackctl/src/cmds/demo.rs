use std::env;

use clap::{Args, Subcommand};
use comfy_table::{presets::NOTHING, ContentArrangement, Row, Table};
use stackable::{
    constants::DEFAULT_LOCAL_CLUSTER_NAME,
    platform::demo::{DemoList, DemoListError},
    types::{
        IntoParameters, IntoParametersError, IntoPathsOrUrls, ParsePathsOrUrls, PathOrUrlParseError,
    },
    utils::read::ReadError,
};
use thiserror::Error;

use crate::{
    cli::{Cli, ClusterType, OutputType},
    constants::{DEMO_FILES_ENV_KEY, REMOTE_DEMO_FILE, REMOTE_STACK_FILE, STACK_FILES_ENV_KEY},
};

const REMOTE_DEMO_FILE: &str =
    "https://raw.githubusercontent.com/stackabletech/stackablectl/main/demos/demos-v2.yaml";

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
pub enum DemoError {
    #[error("read error: {0}")]
    ReadError(#[from] ReadError),

    #[error("yaml error: {0}")]
    YamlError(#[from] serde_yaml::Error),

    #[error("json error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("no demo with name '{0}'")]
    NoSuchDemo(String),

    #[error("failed to convert input parameters to validated parameters: {0}")]
    IntoParametersError(#[from] IntoParametersError),

    #[error("demo list error: {0}")]
    DemoListError(#[from] DemoListError),

    #[error("path/url parse error: {0}")]
    PathOrUrlParseError(#[from] PathOrUrlParseError),
}

impl DemoArgs {
    pub async fn run(&self, common_args: &Cli) -> Result<String, DemoError> {
        // Build demo list based on the (default) remote demo file, and additional files provided by the
        // STACKABLE_ADDITIONAL_DEMO_FILES env variable or the --additional-demo-files CLI argument.

        let env_files = match env::var(ADDITIONAL_DEMO_FILES_ENV_KEY) {
            Ok(env_files) => env_files.parse_paths_or_urls()?,
            Err(_) => vec![],
        };

        let arg_files = common_args
            .additional_demo_files
            .clone()
            .into_paths_or_urls()?;

        let list = DemoList::build(REMOTE_DEMO_FILE, env_files, arg_files).await?;

        match &self.subcommand {
            DemoCommands::List(args) => list_cmd(args, list).await,
            DemoCommands::Describe(args) => describe_cmd(args, list).await,
            DemoCommands::Install(args) => install_cmd(args, list),
            DemoCommands::Uninstall(args) => uninstall_cmd(args, list),
        }
    }
}

/// Print out a list of demos, either as a table (plain), JSON or YAML
async fn list_cmd(args: &DemoListArgs, list: DemoList) -> Result<String, DemoError> {
    match args.output_type {
        OutputType::Plain => {
            let mut table = Table::new();

            table
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_header(vec!["NAME", "STACK", "DESCRIPTION"]);

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
async fn describe_cmd(args: &DemoDescribeArgs, list: DemoList) -> Result<String, DemoError> {
    let demo = list
        .get(&args.demo_name)
        .ok_or(DemoError::NoSuchDemo(args.demo_name.clone()))?;

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
fn install_cmd(args: &DemoInstallArgs, list: DemoList) -> Result<String, DemoError> {
    let demo = list
        .get(&args.demo_name)
        .ok_or(DemoError::NoSuchDemo(args.demo_name.clone()))?;

    let parameters = args.parameters.clone().into_params(&demo.parameters)?;

    todo!()
}

fn uninstall_cmd(args: &DemoUninstallArgs, list: DemoList) -> Result<String, DemoError> {
    todo!()
}
