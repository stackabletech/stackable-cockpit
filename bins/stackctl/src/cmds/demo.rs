use std::{collections::HashMap, env, path::PathBuf};

use clap::{Args, Subcommand};
use comfy_table::{presets::NOTHING, ContentArrangement, Row, Table};
use stackable::{
    constants::DEFAULT_LOCAL_CLUSTER_NAME,
    types::demos::{DemoSpecV2, DemosV2},
};
use thiserror::Error;

use crate::{
    cli::{Cli, ClusterType, OutputType},
    constants::ADDITIONAL_DEMO_FILES_ENV_KEY,
    utils::{read_from_file_or_url, string_to_paths, PathParseError, ReadError},
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

    #[error("path parse error: {0}")]
    PathParseError(#[from] PathParseError),

    #[error("no demo with name '{0}'")]
    NoSuchDemo(String),
}

impl DemoArgs {
    pub async fn run(&self, common_args: &Cli) -> Result<String, DemoError> {
        // Build demo list based on the (default) remote demo file, and additional files provided by the
        // STACKABLE_ADDITIONAL_DEMO_FILES env variable or the --additional-demo-files CLI argument.
        let list = DemoList::build(&common_args.additional_demo_files).await?;

        match &self.subcommand {
            DemoCommands::List(args) => list_cmd(args, list).await,
            DemoCommands::Describe(args) => describe_cmd(args, list).await,
            DemoCommands::Install(args) => install_cmd(args, list),
            DemoCommands::Uninstall(args) => uninstall_cmd(args, list),
        }
    }
}

#[derive(Debug)]
pub struct DemoList(HashMap<String, DemoSpecV2>);

impl DemoList {
    pub async fn build(additional_demo_files: &Vec<String>) -> Result<Self, DemoError> {
        let mut map = HashMap::new();

        // First load the remote demo file
        let demos = get_remote_demos().await?;
        for (demo_name, demo) in demos.iter() {
            map.insert(demo_name.to_owned(), demo.to_owned());
        }

        // After that, the STACKABLE_ADDITIONAL_DEMO_FILES env var is used
        if let Ok(paths_string) = env::var(ADDITIONAL_DEMO_FILES_ENV_KEY) {
            let paths = string_to_paths(paths_string)?;

            for path in paths {
                let demos = get_local_demos(path).await?;
                for (demo_name, demo) in demos.iter() {
                    map.insert(demo_name.to_owned(), demo.to_owned());
                }
            }
        }

        // Lastly, the CLI argument --additional-demo-files is used
        for path in additional_demo_files {
            let demos = get_local_demos(path.into()).await?;
            for (demo_name, demo) in demos.iter() {
                map.insert(demo_name.to_owned(), demo.to_owned());
            }
        }

        Ok(Self(map))
    }

    /// Get a demo by name
    pub fn get<T>(&self, demo_name: T) -> Option<&DemoSpecV2>
    where
        T: Into<String>,
    {
        self.0.get(&demo_name.into())
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

            for (demo_name, demo_spec) in list.0.iter() {
                let row = Row::from(vec![
                    demo_name.clone(),
                    demo_spec.stack.clone(),
                    demo_spec.description.clone(),
                ]);
                table.add_row(row);
            }

            Ok(table.to_string())
        }
        OutputType::Json => Ok(serde_json::to_string(&list.0)?),
        OutputType::Yaml => Ok(serde_yaml::to_string(&list.0)?),
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

    todo!()
}

fn uninstall_cmd(args: &DemoUninstallArgs, list: DemoList) -> Result<String, DemoError> {
    todo!()
}

async fn get_remote_demos() -> Result<DemosV2, DemoError> {
    let content = read_from_file_or_url(REMOTE_DEMO_FILE).await?;
    let demos = serde_yaml::from_str::<DemosV2>(&content)?;

    Ok(demos)
}

async fn get_local_demos(path: PathBuf) -> Result<DemosV2, DemoError> {
    let content = read_from_file_or_url(path).await?;
    let demos = serde_yaml::from_str(&content)?;

    Ok(demos)
}
