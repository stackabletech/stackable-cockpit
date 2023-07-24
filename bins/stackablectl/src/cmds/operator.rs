use std::collections::HashMap;

use clap::{Args, Subcommand};
use indexmap::IndexMap;
use semver::Version;
use serde::Serialize;
use thiserror::Error;
use tracing::{debug, info, instrument};

use stackable::{
    constants::{HELM_REPO_NAME_DEV, HELM_REPO_NAME_STABLE, HELM_REPO_NAME_TEST},
    helm::{self, HelmError, HelmRelease, HelmRepo},
    platform::operator::{OperatorSpec, VALID_OPERATORS},
    utils,
};

use crate::{
    cli::{Cli, CommonClusterArgs, CommonClusterArgsError, OutputType},
    output::{ResultOutput, TabledOutput},
    utils::{helm_repo_name_to_repo_url, InvalidRepoNameError},
};

#[derive(Debug, Args)]
pub struct OperatorArgs {
    #[command(subcommand)]
    subcommand: OperatorCommands,
}

#[derive(Debug, Subcommand)]
pub enum OperatorCommands {
    /// List available (or installed) operators
    #[command(alias("ls"))]
    List(OperatorListArgs),

    /// Print out detailed operator information
    #[command(alias("desc"))]
    Describe(OperatorDescribeArgs),

    /// Install one or more operators
    #[command(aliases(["i", "in"]))]
    Install(OperatorInstallArgs),

    /// Uninstall one or more operators
    #[command(aliases(["rm", "un"]))]
    Uninstall(OperatorUninstallArgs),

    /// List installed operator (same as list -i)
    Installed(OperatorInstalledArgs),
}

#[derive(Debug, Args)]
pub struct OperatorListArgs {
    #[arg(short, long = "output", value_enum, default_value_t = Default::default())]
    output_type: OutputType,
}

#[derive(Debug, Args)]
pub struct OperatorDescribeArgs {
    /// Operator to describe
    #[arg(name = "OPERATOR", required = true)]
    operator_name: String,

    #[arg(short, long = "output", value_enum, default_value_t = Default::default())]
    output_type: OutputType,
}

#[derive(Debug, Args)]
pub struct OperatorInstallArgs {
    /// Operator(s) to install
    #[arg(name = "OPERATORS", required = true)]
    #[arg(long_help = "Operator(s) to install

Must have the form 'name[=version]'. If no version is specified the latest
nightly version - build from the main branch - will be used. Possible valid
values are:

- superset
- superset=0.3.0
- superset=0.3.0-nightly
- superset=0.3.0-pr123

Use \"stackablectl operator list\" to list available versions for all operators
Use \"stackablectl operator describe <OPERATOR>\" to get available versions for one operator")]
    operators: Vec<OperatorSpec>,

    #[command(flatten)]
    local_cluster: CommonClusterArgs,
}

#[derive(Debug, Args)]
pub struct OperatorUninstallArgs {
    /// One or more operators to uninstall
    #[arg(required = true)]
    operators: Vec<OperatorSpec>,
}

#[derive(Debug, Args)]
pub struct OperatorInstalledArgs {
    #[arg(short, long = "output", value_enum, default_value_t = Default::default())]
    output_type: OutputType,
}

#[derive(Debug, Error)]
pub enum OperatorCmdError {
    #[error("invalid repo name")]
    InvalidRepoNameError(#[from] InvalidRepoNameError),

    #[error("unknown repo name: {0}")]
    UnknownRepoNameError(String),

    #[error("Helm error")]
    HelmError(#[from] HelmError),

    #[error("cluster argument error")]
    CommonClusterArgsError(#[from] CommonClusterArgsError),

    #[error("semver parse error")]
    SemVerParseError(#[from] semver::Error),

    #[error("unable to format yaml output")]
    YamlOutputFormatError(#[from] serde_yaml::Error),

    #[error("unable to format json output")]
    JsonOutputFormatError(#[from] serde_json::Error),
}

/// This list contains a list of operator version grouped by stable, test and
/// dev lines. The lines can be accessed by the globally defined constants like
/// [`HELM_REPO_NAME_STABLE`].
#[derive(Debug, Serialize)]
pub struct OperatorVersionList(HashMap<String, Vec<String>>);

impl OperatorArgs {
    pub async fn run(&self, common_args: &Cli) -> Result<String, OperatorCmdError> {
        match &self.subcommand {
            OperatorCommands::List(args) => list_cmd(args, common_args).await,
            OperatorCommands::Describe(args) => describe_cmd(args).await,
            OperatorCommands::Install(args) => install_cmd(args, common_args).await,
            OperatorCommands::Uninstall(args) => uninstall_cmd(args, common_args),
            OperatorCommands::Installed(args) => installed_cmd(args, common_args),
        }
    }
}

impl ResultOutput for IndexMap<String, OperatorVersionList> {
    const EMPTY_MESSAGE: &'static str = "No operators";
    type Error = OperatorCmdError;
}

impl TabledOutput for IndexMap<String, OperatorVersionList> {
    const COLUMNS: &'static [&'static str] = &["#", "OPERATOR", "STABLE VERSIONS"];
    type Row = Vec<String>;

    fn rows(&self) -> Vec<Self::Row> {
        self.iter()
            .enumerate()
            .map(|(index, (operator_name, versions))| {
                let versions_string = versions
                    .0
                    .get(HELM_REPO_NAME_STABLE)
                    .map_or("".into(), |v| v.join(", "));

                vec![
                    (index + 1).to_string(),
                    operator_name.clone(),
                    versions_string,
                ]
            })
            .collect()
    }
}

impl ResultOutput for OperatorVersionList {
    type Error = OperatorCmdError;
}

impl TabledOutput for OperatorVersionList {
    type Row = Vec<String>;

    fn rows(&self) -> Vec<Self::Row> {
        let stable_versions_string = self
            .0
            .get(HELM_REPO_NAME_STABLE)
            .map_or("".into(), |v| v.join(", "));

        let test_versions_string = self
            .0
            .get(HELM_REPO_NAME_TEST)
            .map_or("".into(), |v| v.join(", "));

        let dev_versions_string = self
            .0
            .get(HELM_REPO_NAME_DEV)
            .map_or("".into(), |v| v.join(", "));

        vec![
            vec!["STABLE VERSIONS".into(), stable_versions_string],
            vec!["TEST VERSIONS".into(), test_versions_string],
            vec!["DEV VERSIONS".into(), dev_versions_string],
        ]
    }
}

impl ResultOutput for IndexMap<String, HelmRelease> {
    type Error = OperatorCmdError;
}

impl TabledOutput for IndexMap<String, HelmRelease> {
    const COLUMNS: &'static [&'static str] =
        &["OPERATOR", "VERSION", "NAMESPACE", "STATUS", "LAST UPDATED"];
    type Row = Vec<String>;

    fn rows(&self) -> Vec<Self::Row> {
        self.into_iter()
            .map(|(release_name, release)| {
                vec![
                    release_name.clone(),
                    release.version.clone(),
                    release.namespace.clone(),
                    release.status.clone(),
                    release.last_updated.clone(),
                ]
            })
            .collect()
    }
}

#[instrument]
async fn list_cmd(args: &OperatorListArgs, common_args: &Cli) -> Result<String, OperatorCmdError> {
    debug!("Listing operators");

    // Build map which maps Helm repo name to Helm repo URL
    let helm_index_files = build_helm_index_file_list().await?;

    // Iterate over all valid operators and create a list of versions grouped
    // by stable, test and dev lines
    let versions_list = build_versions_list(&helm_index_files)?;

    Ok(versions_list.output(args.output_type)?)
}

#[instrument]
async fn describe_cmd(args: &OperatorDescribeArgs) -> Result<String, OperatorCmdError> {
    debug!("Describing operator {}", args.operator_name);

    // Build map which maps Helm repo name to Helm repo URL
    let helm_index_files = build_helm_index_file_list().await?;

    // Create a list of versions for this operator
    let versions_list = build_versions_list_for_operator(&args.operator_name, &helm_index_files)?;

    Ok(versions_list.output(args.output_type)?)
}

#[instrument]
async fn install_cmd(
    args: &OperatorInstallArgs,
    common_args: &Cli,
) -> Result<String, OperatorCmdError> {
    info!("Installing operator(s)");

    println!(
        "Installing {} {}",
        args.operators.len(),
        if args.operators.len() == 1 {
            "operator"
        } else {
            "operators"
        }
    );

    args.local_cluster.install_if_needed(None).await?;

    for operator in &args.operators {
        println!("Installing {} operator", operator.name);
        operator.install(&common_args.operator_namespace)?;
        println!("Installed {} operator", operator.name)
    }

    Ok(format!(
        "Installed {} {}",
        args.operators.len(),
        if args.operators.len() == 1 {
            "operator"
        } else {
            "operators"
        }
    ))
}

#[instrument]
fn uninstall_cmd(
    args: &OperatorUninstallArgs,
    common_args: &Cli,
) -> Result<String, OperatorCmdError> {
    info!("Uninstalling operator(s)");

    for operator in &args.operators {
        operator.uninstall(&common_args.operator_namespace)?;
    }

    Ok(format!(
        "Uninstalled {} {}",
        args.operators.len(),
        if args.operators.len() == 1 {
            "operator"
        } else {
            "operators"
        }
    ))
}

#[instrument]
fn installed_cmd(
    args: &OperatorInstalledArgs,
    common_args: &Cli,
) -> Result<String, OperatorCmdError> {
    debug!("Listing installed operators");

    type ReleaseList = IndexMap<String, HelmRelease>;

    let installed: ReleaseList = helm::list_releases(&common_args.operator_namespace)?
        .into_iter()
        .filter(|release| {
            VALID_OPERATORS
                .iter()
                .any(|valid| release.name == utils::operator_chart_name(valid))
        })
        .map(|release| (release.name.clone(), release))
        .collect();

    Ok(installed.output(args.output_type)?)
}

/// Builds a map which maps Helm repo name to Helm repo URL.
#[instrument]
async fn build_helm_index_file_list<'a>() -> Result<HashMap<&'a str, HelmRepo>, OperatorCmdError> {
    debug!("Building Helm index file list");

    let mut helm_index_files = HashMap::new();

    for helm_repo_name in [
        HELM_REPO_NAME_STABLE,
        HELM_REPO_NAME_TEST,
        HELM_REPO_NAME_DEV,
    ] {
        let helm_repo_url = helm_repo_name_to_repo_url(helm_repo_name)?;
        helm_index_files.insert(helm_repo_name, helm::get_helm_index(helm_repo_url).await?);
    }

    Ok(helm_index_files)
}

/// Iterates over all valid operators and creates a list of versions grouped
/// by stable, test and dev lines based on the list of Helm repo index files.
#[instrument]
fn build_versions_list(
    helm_index_files: &HashMap<&str, HelmRepo>,
) -> Result<IndexMap<String, OperatorVersionList>, OperatorCmdError> {
    debug!("Building versions list");

    let mut versions_list = IndexMap::new();

    for operator in VALID_OPERATORS {
        for (helm_repo_name, helm_repo_index_file) in helm_index_files {
            let versions = list_operator_versions_from_repo(operator, helm_repo_index_file)?;
            let entry = versions_list.entry(operator.to_string());
            let entry = entry.or_insert(OperatorVersionList(HashMap::new()));
            entry.0.insert(helm_repo_name.to_string(), versions);
        }
    }

    Ok(versions_list)
}

/// Builds a list of versions for one operator (by name) which is grouped by
/// stable, test and dev lines based on the list of Helm repo index files.
#[instrument]
fn build_versions_list_for_operator<T>(
    operator_name: T,
    helm_index_files: &HashMap<&str, HelmRepo>,
) -> Result<OperatorVersionList, OperatorCmdError>
where
    T: AsRef<str> + std::fmt::Debug,
{
    debug!(
        "Build versions list for operator {}",
        operator_name.as_ref()
    );

    let mut versions_list = OperatorVersionList(HashMap::new());
    let operator_name = operator_name.as_ref();

    for (helm_repo_name, helm_repo_index_file) in helm_index_files {
        let versions = list_operator_versions_from_repo(operator_name, helm_repo_index_file)?;
        versions_list.0.insert(helm_repo_name.to_string(), versions);
    }

    Ok(versions_list)
}

/// Builds a list of operator versions based on the provided Helm repo.
#[instrument]
fn list_operator_versions_from_repo<T>(
    operator_name: T,
    helm_repo: &HelmRepo,
) -> Result<Vec<String>, OperatorCmdError>
where
    T: AsRef<str> + std::fmt::Debug,
{
    debug!("Listing operator versions from repo");

    let operator_name = utils::operator_chart_name(operator_name.as_ref());

    match helm_repo.entries.get(&operator_name) {
        Some(entries) => {
            let mut versions = entries
                .iter()
                .map(|e| Version::parse(&e.version))
                .map_while(|r| match r {
                    Ok(v) => Some(v),
                    Err(_) => None,
                })
                .map(|v| v.to_string())
                .collect::<Vec<String>>();

            versions.sort();
            Ok(versions)
        }
        None => Ok(vec![]),
    }
}
