use std::collections::HashMap;

use clap::{Args, Subcommand};
use comfy_table::{
    presets::{NOTHING, UTF8_FULL},
    ContentArrangement, Table,
};
use indexmap::IndexMap;
use semver::Version;
use serde::Serialize;
use snafu::{ResultExt, Snafu};
use tracing::{debug, info, instrument};

use stackable_cockpit::{
    constants::{
        DEFAULT_OPERATOR_NAMESPACE, HELM_REPO_NAME_DEV, HELM_REPO_NAME_STABLE, HELM_REPO_NAME_TEST,
    },
    helm::{self, HelmError, HelmRelease, HelmRepo},
    platform::operator::{OperatorSpec, VALID_OPERATORS},
    utils,
};

use crate::{
    args::{CommonClusterArgs, CommonClusterArgsError},
    cli::{Cli, OutputType},
    utils::{helm_repo_name_to_repo_url, InvalidRepoNameError},
};

#[derive(Debug, Args)]
pub struct OperatorArgs {
    #[command(subcommand)]
    subcommand: OperatorCommands,
}

#[derive(Debug, Subcommand)]
pub enum OperatorCommands {
    /// List available operators
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

    /// List installed operators
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

    /// Namespace in the cluster used to deploy the operators
    #[arg(long, default_value = DEFAULT_OPERATOR_NAMESPACE, visible_aliases(["operator-ns"]))]
    pub operator_namespace: String,

    #[command(flatten)]
    local_cluster: CommonClusterArgs,
}

#[derive(Debug, Args)]
pub struct OperatorUninstallArgs {
    /// One or more operators to uninstall
    #[arg(required = true)]
    operators: Vec<OperatorSpec>,

    /// Namespace in the cluster used to deploy the operators
    #[arg(long, default_value = DEFAULT_OPERATOR_NAMESPACE, visible_aliases(["operator-ns"]))]
    pub operator_namespace: String,
}

#[derive(Debug, Args)]
pub struct OperatorInstalledArgs {
    #[arg(short, long = "output", value_enum, default_value_t = Default::default())]
    output_type: OutputType,

    /// Namespace in the cluster used to deploy the operators
    #[arg(long, default_value = DEFAULT_OPERATOR_NAMESPACE, visible_aliases(["operator-ns"]))]
    pub operator_namespace: String,
}

#[derive(Debug, Snafu)]
pub enum OperatorCmdError {
    #[snafu(display("invalid repo name"))]
    InvalidRepoNameError { source: InvalidRepoNameError },

    #[snafu(display("unknown repo name: {name}"))]
    UnknownRepoNameError { name: String },

    #[snafu(display("Helm error"))]
    HelmError { source: HelmError },

    #[snafu(display("cluster argument error"))]
    CommonClusterArgsError { source: CommonClusterArgsError },

    #[snafu(display("semver parse error"))]
    SemVerParseError { source: semver::Error },

    #[snafu(display("unable to format yaml output"))]
    YamlOutputFormatError { source: serde_yaml::Error },

    #[snafu(display("unable to format json output"))]
    JsonOutputFormatError { source: serde_json::Error },
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

#[instrument]
async fn list_cmd(args: &OperatorListArgs, common_args: &Cli) -> Result<String, OperatorCmdError> {
    debug!("Listing operators");

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
                .set_header(vec!["#", "OPERATOR", "STABLE VERSIONS"])
                .load_preset(UTF8_FULL);

            for (index, (operator_name, versions)) in versions_list.iter().enumerate() {
                let versions_string = match versions.0.get(HELM_REPO_NAME_STABLE) {
                    Some(v) => v.join(", "),
                    None => "".into(),
                };
                table.add_row(vec![
                    (index + 1).to_string(),
                    operator_name.clone(),
                    versions_string,
                ]);
            }

            Ok(table.to_string())
        }
        OutputType::Json => {
            Ok(serde_json::to_string(&versions_list).context(JsonOutputFormatSnafu)?)
        }
        OutputType::Yaml => {
            Ok(serde_yaml::to_string(&versions_list).context(YamlOutputFormatSnafu)?)
        }
    }
}

#[instrument]
async fn describe_cmd(args: &OperatorDescribeArgs) -> Result<String, OperatorCmdError> {
    debug!("Describing operator {}", args.operator_name);

    // Build map which maps Helm repo name to Helm repo URL
    let helm_index_files = build_helm_index_file_list().await?;

    // Create a list of versions for this operator
    let versions_list = build_versions_list_for_operator(&args.operator_name, &helm_index_files)?;

    match args.output_type {
        OutputType::Plain => {
            let stable_versions_string = match versions_list.0.get(HELM_REPO_NAME_STABLE) {
                Some(v) => v.join(", "),
                None => "".into(),
            };

            let test_versions_string = match versions_list.0.get(HELM_REPO_NAME_TEST) {
                Some(v) => v.join(", "),
                None => "".into(),
            };

            let dev_versions_string = match versions_list.0.get(HELM_REPO_NAME_DEV) {
                Some(v) => v.join(", "),
                None => "".into(),
            };

            let mut table = Table::new();

            table
                .set_content_arrangement(ContentArrangement::Dynamic)
                .load_preset(NOTHING)
                .add_row(vec!["OPERATOR", &args.operator_name.to_string()])
                .add_row(vec!["STABLE VERSIONS", stable_versions_string.as_str()])
                .add_row(vec!["TEST VERSIONS", test_versions_string.as_str()])
                .add_row(vec!["DEV VERSIONS", dev_versions_string.as_str()]);

            Ok(table.to_string())
        }
        OutputType::Json => serde_json::to_string(&versions_list).context(JsonOutputFormatSnafu),
        OutputType::Yaml => serde_yaml::to_string(&versions_list).context(YamlOutputFormatSnafu),
    }
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

    args.local_cluster
        .install_if_needed(None)
        .await
        .context(CommonClusterArgsSnafu)?;

    for operator in &args.operators {
        println!("Installing {} operator", operator.name);

        match operator.install(&args.operator_namespace) {
            Ok(_) => println!("Installed {} operator", operator.name),
            Err(err) => {
                return Err(OperatorCmdError::HelmError { source: err });
            }
        };
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
        operator
            .uninstall(&args.operator_namespace)
            .context(HelmSnafu)?;
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

    let installed: ReleaseList = helm::list_releases(&args.operator_namespace)
        .context(HelmSnafu)?
        .into_iter()
        .filter(|release| {
            VALID_OPERATORS
                .iter()
                .any(|valid| release.name == utils::operator_chart_name(valid))
        })
        .map(|release| (release.name.clone(), release))
        .collect();

    match args.output_type {
        OutputType::Plain => {
            if installed.is_empty() {
                return Ok("No installed operators".into());
            }

            let mut table = Table::new();

            table
                .set_content_arrangement(ContentArrangement::Dynamic)
                .load_preset(UTF8_FULL)
                .set_header(vec![
                    "OPERATOR",
                    "VERSION",
                    "NAMESPACE",
                    "STATUS",
                    "LAST UPDATED",
                ]);

            for (release_name, release) in installed {
                table.add_row(vec![
                    release_name,
                    release.version,
                    release.namespace,
                    release.status,
                    release.last_updated,
                ]);
            }

            Ok(table.to_string())
        }
        OutputType::Json => Ok(serde_json::to_string(&installed).context(JsonOutputFormatSnafu)?),
        OutputType::Yaml => Ok(serde_yaml::to_string(&installed).context(YamlOutputFormatSnafu)?),
    }
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
        let helm_repo_url =
            helm_repo_name_to_repo_url(helm_repo_name).context(InvalidRepoNameSnafu)?;

        helm_index_files.insert(
            helm_repo_name,
            helm::get_helm_index(helm_repo_url)
                .await
                .context(HelmSnafu)?,
        );
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
