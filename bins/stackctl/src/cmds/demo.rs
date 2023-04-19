use std::{collections::HashMap, env};

use clap::{Args, Subcommand};
use stackable::{
    constants::DEFAULT_LOCAL_CLUSTER_NAME,
    types::demo::{DemoSpecV2, DemosV2},
};
use thiserror::Error;

use crate::{
    cli::{ClusterType, OutputType},
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
    /// List deployed services
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

#[derive(Debug, Error)]
pub enum DemoError {
    #[error("read error: {0}")]
    ReadError(#[from] ReadError),

    #[error("yaml error: {0}")]
    YamlError(#[from] serde_yaml::Error),

    #[error("path parse error: {0}")]
    PathParseError(#[from] PathParseError),
}

impl DemoArgs {
    pub fn run(&self) -> Result<String, DemoError> {
        match &self.subcommand {
            DemoCommands::List(args) => list_cmd(args),
            DemoCommands::Describe(args) => describe_cmd(args),
            DemoCommands::Install(args) => install_cmd(args),
        }
    }
}

pub struct DemoList(HashMap<String, DemoSpecV2>);

impl DemoList {
    pub async fn build(additional_demo_files: Vec<String>) -> Result<Self, DemoError> {
        let mut list = HashMap::new();

        // First load the remote demo file
        let demos = get_remote_demos().await?;
        for (demo_name, demo) in demos.iter() {
            list.insert(demo_name.to_owned(), demo.to_owned());
        }

        // After that, the STACKABLE_ADDITIONAL_DEMO_FILES env var is used
        if let Ok(paths_string) = env::var(ADDITIONAL_DEMO_FILES_ENV_KEY) {
            let paths = string_to_paths(paths_string)?;
        }

        // Lastly, the CLI argument --additional-demo-files is used

        Ok(Self(list))
    }
}

fn list_cmd(args: &DemoListArgs) -> Result<String, DemoError> {
    todo!()
}

fn describe_cmd(args: &DemoDescribeArgs) -> Result<String, DemoError> {
    todo!()
}

fn install_cmd(args: &DemoInstallArgs) -> Result<String, DemoError> {
    todo!()
}

async fn get_remote_demos() -> Result<DemosV2, DemoError> {
    let content = read_from_file_or_url(REMOTE_DEMO_FILE).await?;
    let demos = serde_yaml::from_str::<DemosV2>(&content)?;

    Ok(demos)
}
