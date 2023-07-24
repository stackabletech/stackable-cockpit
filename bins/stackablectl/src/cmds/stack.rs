use clap::{Args, Subcommand};
use comfy_table::{presets::NOTHING, ContentArrangement, Table};
use thiserror::Error;
use tracing::{info, instrument};

use stackable::{
    common::ListError,
    platform::{
        release::ReleaseList,
        stack::{StackError, StackList, StackSpecV2},
    },
    utils::path::PathOrUrlParseError,
};

use crate::{
    cli::{CacheSettingsError, Cli, CommonClusterArgs, CommonClusterArgsError, OutputType},
    output::{ResultOutput, TabledOutput},
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

    /// List of parameters to use when installing the stack
    #[arg(short, long)]
    stack_parameters: Vec<String>,

    /// List of parameters to use when installing the stack
    #[arg(short, long)]
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
}

#[derive(Debug, Error)]
pub enum StackCmdError {
    #[error("path/url parse error")]
    PathOrUrlParseError(#[from] PathOrUrlParseError),

    #[error("unable to format yaml output")]
    YamlOutputFormatError(#[from] serde_yaml::Error),

    #[error("unable to format json output")]
    JsonOutputFormatError(#[from] serde_json::Error),

    #[error("stack error")]
    StackError(#[from] StackError),

    #[error("list error")]
    ListError(#[from] ListError),

    #[error("cache settings resolution error")]
    CacheSettingsError(#[from] CacheSettingsError),

    #[error("cluster argument error")]
    CommonClusterArgsError(#[from] CommonClusterArgsError),

    #[error("no stack with name '{0}'")]
    NoSuchStack(String),
}

impl ResultOutput for StackList {
    type Error = StackCmdError;
}

impl TabledOutput for StackList {
    const COLUMNS: &'static [&'static str] = &["#", "STACK", "RELEASE", "DESCRIPTION"];
    type Row = Vec<String>;

    fn rows(&self) -> Vec<Self::Row> {
        self.inner()
            .iter()
            .enumerate()
            .map(|(index, (stack_name, stack))| {
                vec![
                    (index + 1).to_string(),
                    stack_name.clone(),
                    stack.release.clone(),
                    stack.description.clone(),
                ]
            })
            .collect()
    }
}

impl ResultOutput for StackSpecV2 {
    type Error = StackCmdError;
}

impl TabledOutput for StackSpecV2 {
    type Row = Vec<String>;

    fn rows(&self) -> Vec<Self::Row> {
        let mut parameter_table = Table::new();

        parameter_table
            .set_header(vec!["NAME", "DESCRIPTION", "DEFAULT VALUE"])
            .set_content_arrangement(ContentArrangement::Dynamic)
            .load_preset(NOTHING);

        for parameter in &self.parameters {
            parameter_table.add_row(vec![
                parameter.name.clone(),
                parameter.description.clone(),
                parameter.default.clone(),
            ]);
        }

        vec![
            vec!["DESCRIPTION".into(), self.description.clone()],
            vec!["RELEASE".into(), self.release.clone()],
            vec!["OPERATORS".into(), self.operators.join(", ")],
            vec!["LABELS".into(), self.labels.join(", ")],
            vec!["PARAMETERS".into(), parameter_table.to_string()],
        ]
    }
}

impl StackArgs {
    pub async fn run(&self, common_args: &Cli) -> Result<String, StackCmdError> {
        let files = common_args.get_stack_files()?;
        let stack_list = StackList::build(&files, &common_args.cache_settings()?).await?;

        match &self.subcommand {
            StackCommands::List(args) => list_cmd(args, stack_list),
            StackCommands::Describe(args) => describe_cmd(args, stack_list),
            StackCommands::Install(args) => install_cmd(args, common_args, stack_list).await,
        }
    }
}

#[instrument]
fn list_cmd(args: &StackListArgs, stack_list: StackList) -> Result<String, StackCmdError> {
    info!("Listing stacks");

    Ok(stack_list.output(args.output_type)?)
}

#[instrument]
fn describe_cmd(args: &StackDescribeArgs, stack_list: StackList) -> Result<String, StackCmdError> {
    info!("Describing stack");

    let stack = stack_list
        .get(&args.stack_name)
        .ok_or(StackCmdError::NoSuchStack(args.stack_name.clone()))?;

    Ok(stack.output(args.output_type)?)
}

#[instrument]
async fn install_cmd(
    args: &StackInstallArgs,
    common_args: &Cli,
    stack_list: StackList,
) -> Result<String, StackCmdError> {
    info!("Installing stack");

    let files = common_args.get_release_files()?;
    let release_list = ReleaseList::build(&files, &common_args.cache_settings()?).await?;

    // Install local cluster if needed
    args.local_cluster.install_if_needed(None).await?;

    match stack_list.get(&args.stack_name) {
        Some(stack_spec) => {
            // Install the stack
            stack_spec.install(release_list, &common_args.operator_namespace)?;

            // Install stack manifests
            stack_spec
                .install_stack_manifests(&args.stack_parameters, &common_args.operator_namespace)
                .await?;

            Ok(format!("Install stack {}", args.stack_name))
        }
        None => Ok("No such stack".into()),
    }
}
