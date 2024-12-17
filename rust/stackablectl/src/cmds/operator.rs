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
    helm::{self, Release, Repository},
    platform::{namespace, operator},
    utils::{
        self,
        k8s::{self, Client},
    },
};

use crate::{
    args::{CommonClusterArgs, CommonClusterArgsError},
    cli::{Cli, OutputType},
    utils::{helm_repo_name_to_repo_url, InvalidRepoNameError},
};

const INSTALL_AFTER_HELP_TEXT: &str = "Examples:

Use \"stackablectl operator install <OPERATOR> -c <OPTION>\" to create a local cluster";

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
    #[command(aliases(["i", "in"]), after_help = INSTALL_AFTER_HELP_TEXT)]
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
    operators: Vec<operator::OperatorSpec>,

    /// Namespace in the cluster used to deploy the operators
    #[arg(long, default_value = DEFAULT_OPERATOR_NAMESPACE, visible_aliases(["operator-ns"]))]
    pub operator_namespace: String,

    #[command(flatten)]
    local_cluster: CommonClusterArgs,

    /// TODO
    #[arg(long, long_help = "TODO")]
    use_registry: bool,
}

#[derive(Debug, Args)]
pub struct OperatorUninstallArgs {
    /// One or more operators to uninstall
    #[arg(required = true)]
    operators: Vec<operator::OperatorSpec>,

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
pub enum CmdError {
    #[snafu(display("invalid repository name"))]
    InvalidRepoName { source: InvalidRepoNameError },

    #[snafu(display("invalid semantic helm chart version {version:?}"))]
    InvalidHelmChartVersion {
        source: semver::Error,
        version: String,
    },

    #[snafu(display("unknown repository name '{name}'"))]
    UnknownRepoName { name: String },

    #[snafu(display("Helm error"))]
    HelmError { source: helm::Error },

    #[snafu(display("cluster argument error"))]
    CommonClusterArgs { source: CommonClusterArgsError },

    #[snafu(display("failed to serialize YAML output"))]
    SerializeYamlOutput { source: serde_yaml::Error },

    #[snafu(display("failed to serialize JSON output"))]
    SerializeJsonOutput { source: serde_json::Error },

    #[snafu(display("failed to create Kubernetes client"))]
    KubeClientCreate { source: k8s::Error },

    #[snafu(display("failed to create namespace '{namespace}'"))]
    NamespaceCreate {
        source: namespace::Error,
        namespace: String,
    },
}

/// This list contains a list of operator version grouped by stable, test and
/// dev lines. The lines can be accessed by the globally defined constants like
/// [`HELM_REPO_NAME_STABLE`].
#[derive(Debug, Serialize)]
pub struct OperatorVersionList(HashMap<String, Vec<String>>);

impl OperatorArgs {
    pub async fn run(&self, cli: &Cli) -> Result<String, CmdError> {
        match &self.subcommand {
            OperatorCommands::List(args) => list_cmd(args, cli).await,
            OperatorCommands::Describe(args) => describe_cmd(args, cli).await,
            OperatorCommands::Install(args) => install_cmd(args, cli).await,
            OperatorCommands::Uninstall(args) => uninstall_cmd(args, cli),
            OperatorCommands::Installed(args) => installed_cmd(args, cli),
        }
    }
}

#[instrument]
async fn list_cmd(args: &OperatorListArgs, cli: &Cli) -> Result<String, CmdError> {
    debug!("Listing operators");

    // Build map which maps Helm repo name to Helm repo URL
    let helm_index_files = build_helm_index_file_list().await?;

    // Iterate over all valid operators and create a list of versions grouped
    // by stable, test and dev lines
    let versions_list = build_versions_list(&helm_index_files)?;

    match args.output_type {
        OutputType::Plain | OutputType::Table => {
            let (arrangement, preset) = match args.output_type {
                OutputType::Plain => (ContentArrangement::Disabled, NOTHING),
                _ => (ContentArrangement::Dynamic, UTF8_FULL),
            };

            let mut table = Table::new();
            table
                .set_header(vec!["#", "OPERATOR", "STABLE VERSIONS"])
                .set_content_arrangement(arrangement)
                .load_preset(preset);

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

            let mut result = cli.result();

            result
                .with_command_hint(
                    "stackablectl operator describe [OPTIONS] <OPERATOR>",
                    "display further information for the specified operator",
                )
                .with_command_hint(
                    "stackablectl operator install [OPTIONS] <OPERATORS>...",
                    "install one or more operators",
                )
                .with_output(table.to_string());

            Ok(result.render())
        }
        OutputType::Json => serde_json::to_string(&versions_list).context(SerializeJsonOutputSnafu),
        OutputType::Yaml => serde_yaml::to_string(&versions_list).context(SerializeYamlOutputSnafu),
    }
}

#[instrument]
async fn describe_cmd(args: &OperatorDescribeArgs, cli: &Cli) -> Result<String, CmdError> {
    debug!("Describing operator {}", args.operator_name);

    // Build map which maps Helm repo name to Helm repo URL
    let helm_index_files = build_helm_index_file_list().await?;

    // Create a list of versions for this operator
    let versions_list = build_versions_list_for_operator(&args.operator_name, &helm_index_files)?;

    match args.output_type {
        OutputType::Plain | OutputType::Table => {
            let arrangement = match args.output_type {
                OutputType::Plain => ContentArrangement::Disabled,
                _ => ContentArrangement::Dynamic,
            };

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
                .set_content_arrangement(arrangement)
                .load_preset(NOTHING)
                .add_row(vec!["OPERATOR", &args.operator_name.to_string()])
                .add_row(vec!["STABLE VERSIONS", stable_versions_string.as_str()])
                .add_row(vec!["TEST VERSIONS", test_versions_string.as_str()])
                .add_row(vec!["DEV VERSIONS", dev_versions_string.as_str()]);

            let mut result = cli.result();

            result
                .with_command_hint(
                    format!("stackablectl operator install {}", args.operator_name),
                    "install the operator",
                )
                .with_command_hint("stackablectl operator list", "list all available operators")
                .with_output(table.to_string());

            Ok(result.render())
        }
        OutputType::Json => serde_json::to_string(&versions_list).context(SerializeJsonOutputSnafu),
        OutputType::Yaml => serde_yaml::to_string(&versions_list).context(SerializeYamlOutputSnafu),
    }
}

#[instrument]
async fn install_cmd(args: &OperatorInstallArgs, cli: &Cli) -> Result<String, CmdError> {
    info!("Installing operator(s)");

    args.local_cluster
        .install_if_needed()
        .await
        .context(CommonClusterArgsSnafu)?;

    let client = Client::new().await.context(KubeClientCreateSnafu)?;

    namespace::create_if_needed(&client, args.operator_namespace.clone())
        .await
        .context(NamespaceCreateSnafu {
            namespace: args.operator_namespace.clone(),
        })?;

    for operator in &args.operators {
        operator
            .install(&args.operator_namespace, args.use_registry)
            .context(HelmSnafu)?;

        println!("Installed {} operator", operator);
    }

    let mut result = cli.result();

    result
        .with_command_hint(
            "stackablectl operator installed [OPTIONS]",
            "list installed operators",
        )
        .with_output(format!(
            "Installed {} {}",
            args.operators.len(),
            if args.operators.len() == 1 {
                "operator"
            } else {
                "operators"
            }
        ));

    Ok(result.render())
}

#[instrument]
fn uninstall_cmd(args: &OperatorUninstallArgs, cli: &Cli) -> Result<String, CmdError> {
    info!("Uninstalling operator(s)");

    for operator in &args.operators {
        operator
            .uninstall(&args.operator_namespace)
            .context(HelmSnafu)?;
    }

    let mut result = cli.result();

    result
        .with_command_hint(
            "stackablectl operator installed [OPTIONS]",
            "list remaining installed operators",
        )
        .with_output(format!(
            "Uninstalled {} {}",
            args.operators.len(),
            if args.operators.len() == 1 {
                "operator"
            } else {
                "operators"
            }
        ));

    Ok(result.render())
}

#[instrument]
fn installed_cmd(args: &OperatorInstalledArgs, cli: &Cli) -> Result<String, CmdError> {
    debug!("Listing installed operators");

    type ReleaseList = IndexMap<String, Release>;

    let installed: ReleaseList = helm::list_releases(&args.operator_namespace)
        .context(HelmSnafu)?
        .into_iter()
        .filter(|release| {
            operator::VALID_OPERATORS
                .iter()
                .any(|valid| release.name == utils::operator_chart_name(valid))
        })
        .map(|release| (release.name.clone(), release))
        .collect();

    match args.output_type {
        OutputType::Plain | OutputType::Table => {
            if installed.is_empty() {
                return Ok("No installed operators".into());
            }

            let (arrangement, preset) = match args.output_type {
                OutputType::Plain => (ContentArrangement::Disabled, NOTHING),
                _ => (ContentArrangement::Dynamic, UTF8_FULL),
            };

            let mut table = Table::new();

            table
                .set_header(vec![
                    "OPERATOR",
                    "VERSION",
                    "NAMESPACE",
                    "STATUS",
                    "LAST UPDATED",
                ])
                .set_content_arrangement(arrangement)
                .load_preset(preset);

            for (release_name, release) in installed {
                table.add_row(vec![
                    release_name,
                    release.version,
                    release.namespace,
                    release.status,
                    release.last_updated,
                ]);
            }

            let mut result = cli.result();

            result
                .with_command_hint(
                    "stackablectl operator install [OPTIONS] <OPERATORS>...",
                    "install one or more additional operators",
                )
                .with_command_hint(
                    "stackablectl operator uninstall [OPTIONS] <OPERATORS>...",
                    "uninstall one or more operators",
                )
                .with_output(table.to_string());

            Ok(result.render())
        }
        OutputType::Json => serde_json::to_string(&installed).context(SerializeJsonOutputSnafu),
        OutputType::Yaml => serde_yaml::to_string(&installed).context(SerializeYamlOutputSnafu),
    }
}

/// Builds a map which maps Helm repo name to Helm repo URL.
#[instrument]
async fn build_helm_index_file_list<'a>() -> Result<HashMap<&'a str, Repository>, CmdError> {
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
    helm_index_files: &HashMap<&str, Repository>,
) -> Result<IndexMap<String, OperatorVersionList>, CmdError> {
    debug!("Building versions list");

    let mut versions_list = IndexMap::new();

    for operator in operator::VALID_OPERATORS {
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
    helm_index_files: &HashMap<&str, Repository>,
) -> Result<OperatorVersionList, CmdError>
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
    helm_repo: &Repository,
) -> Result<Vec<String>, CmdError>
where
    T: AsRef<str> + std::fmt::Debug,
{
    debug!("Listing operator versions from repo");

    let operator_name = utils::operator_chart_name(operator_name.as_ref());

    match helm_repo.entries.get(&operator_name) {
        Some(entries) => {
            let mut versions = entries
                .iter()
                .map(|entry| {
                    Version::parse(&entry.version).with_context(|_| InvalidHelmChartVersionSnafu {
                        version: entry.version.clone(),
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;
            versions.sort();

            Ok(versions.iter().map(|version| version.to_string()).collect())
        }
        None => Ok(vec![]),
    }
}
