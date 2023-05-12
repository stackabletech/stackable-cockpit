use std::collections::HashMap;

use clap::{Args, Subcommand};
use comfy_table::{presets::UTF8_FULL, ContentArrangement, Table};
use indexmap::IndexMap;
use semver::Version;
use snafu::{ResultExt, Snafu};
use stackable::{
    constants::{
        DEFAULT_LOCAL_CLUSTER_NAME, HELM_REPO_NAME_DEV, HELM_REPO_NAME_STABLE, HELM_REPO_NAME_TEST,
    },
    helm::{self, HelmError, HelmRepo},
    platform::operator::{OperatorSpec, VALID_OPERATORS},
};
use tracing::{debug, instrument};

use crate::{
    cli::{ClusterType, OutputType},
    util::{self, InvalidRepoNameError},
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
    /// List only installed operators
    #[arg(short = 'i', long = "installed")]
    list_installed: bool,

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

#[derive(Debug, Snafu)]
pub enum OperatorError {
    #[snafu(display("invalid repo name: {source}"))]
    InvalidRepoNameError { source: InvalidRepoNameError },

    #[snafu(display("unknown repo name: {name}"))]
    UnknownRepoNameError { name: String },

    #[snafu(display("Helm error: {source}"))]
    HelmError { source: HelmError },

    #[snafu(display("semver parse error: {source}"))]
    SemVerParseError { source: semver::Error },
}

/// This list contains a list of operator version grouped by stable, test and
/// dev lines. The lines can be accessed by the globally defined constants like
/// [`HELM_REPO_NAME_STABLE`].
pub struct OperatorVersionList(HashMap<String, Vec<String>>);

impl OperatorArgs {
    pub async fn run(&self) -> Result<String, OperatorError> {
        match &self.subcommand {
            OperatorCommands::List(args) => list_cmd(args).await,
            OperatorCommands::Describe(args) => describe_cmd(args),
            OperatorCommands::Install(args) => install_cmd(args),
            OperatorCommands::Uninstall(args) => uninstall_cmd(args),
            OperatorCommands::Installed(args) => installed_cmd(args),
        }
    }
}

#[instrument]
async fn list_cmd(args: &OperatorListArgs) -> Result<String, OperatorError> {
    debug!("Listing operators");

    // If the user only wnats to list installed operator, use this shortcut
    if args.list_installed {
        return installed_cmd(&OperatorInstalledArgs {
            output_type: args.output_type.clone(),
        });
    }

    // Build map which maps Helm repo name to Helm repo URL
    let helm_index_files = build_helm_index_file_list().await?;

    // Iterate over all valid operators and create a list of versions grouped
    // by stable, test and dev lines
    let versions_list = build_versions_list(&helm_index_files)?;

    match args.output_type {
        OutputType::Plain => {
            let mut table = Table::new();

            table
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_header(vec!["OPERATOR", "STABLE VERSIONS"])
                .load_preset(UTF8_FULL);

            for (operator_name, versions) in versions_list {
                let versions_string = match versions.0.get(HELM_REPO_NAME_STABLE) {
                    Some(v) => v.join(", "),
                    None => "".into(),
                };
                table.add_row(vec![operator_name, versions_string]);
            }

            Ok(table.to_string())
        }
        OutputType::Json => todo!(),
        OutputType::Yaml => todo!(),
    }
}

fn describe_cmd(args: &OperatorDescribeArgs) -> Result<String, OperatorError> {
    todo!()
}

fn install_cmd(args: &OperatorInstallArgs) -> Result<String, OperatorError> {
    todo!()
}

fn uninstall_cmd(args: &OperatorUninstallArgs) -> Result<String, OperatorError> {
    todo!()
}

#[instrument]
fn installed_cmd(args: &OperatorInstalledArgs) -> Result<String, OperatorError> {
    debug!("Listing installed operators");
    todo!()
}

/// Builds a map which maps Helm repo name to Helm repo URL.
#[instrument]
async fn build_helm_index_file_list<'a>() -> Result<HashMap<&'a str, HelmRepo>, OperatorError> {
    debug!("Building Helm index file list");

    let mut helm_index_files = HashMap::new();

    for helm_repo_name in [
        HELM_REPO_NAME_STABLE,
        HELM_REPO_NAME_TEST,
        HELM_REPO_NAME_DEV,
    ] {
        let helm_repo_url =
            util::helm_repo_name_to_repo_url(helm_repo_name).context(InvalidRepoNameSnafu {})?;

        helm_index_files.insert(
            helm_repo_name,
            helm::get_helm_index(helm_repo_url)
                .await
                .context(HelmSnafu {})?,
        );
    }

    Ok(helm_index_files)
}

/// Iterates over all valid operators and creates a list of versions grouped
/// by stable, test and dev lines based on the list of Helm repo index files.
#[instrument]
fn build_versions_list(
    helm_index_files: &HashMap<&str, HelmRepo>,
) -> Result<IndexMap<String, OperatorVersionList>, OperatorError> {
    debug!("Building versions list");

    let mut versions_list = IndexMap::new();

    for operator in VALID_OPERATORS {
        for (helm_repo_name, helm_repo_index_file) in helm_index_files {
            let versions = list_operator_versions_from_repo(operator, &helm_repo_index_file)?;
            let entry = versions_list.entry(operator.to_string());
            let entry = entry.or_insert(OperatorVersionList(HashMap::new()));
            entry.0.insert(helm_repo_name.to_string(), versions);
        }
    }

    Ok(versions_list)
}

/// Builds a list of operator versions based on the provided Helm repo.
#[instrument]
fn list_operator_versions_from_repo<T>(
    operator_name: T,
    helm_repo: &HelmRepo,
) -> Result<Vec<String>, OperatorError>
where
    T: AsRef<str> + std::fmt::Debug,
{
    debug!("Listing operator versions from repo");

    let operator_name = format!("{}-operator", operator_name.as_ref());
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
