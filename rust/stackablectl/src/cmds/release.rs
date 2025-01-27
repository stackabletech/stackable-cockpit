use clap::{Args, Subcommand};
use comfy_table::{
    presets::{NOTHING, UTF8_FULL},
    ContentArrangement, Table,
};
use snafu::{ResultExt, Snafu};
use tracing::{debug, info, instrument};

use stackable_cockpit::{
    common::list,
    constants::DEFAULT_OPERATOR_NAMESPACE,
    platform::{namespace, operator::ChartSourceType, release},
    utils::{
        k8s::{self, Client},
        path::PathOrUrlParseError,
    },
    xfer::{self, cache::Cache},
};

use crate::{
    args::{CommonClusterArgs, CommonClusterArgsError},
    cli::{Cli, OutputType},
};

#[derive(Debug, Args)]
pub struct ReleaseArgs {
    #[command(subcommand)]
    subcommand: ReleaseCommands,
}

#[derive(Debug, Subcommand)]
pub enum ReleaseCommands {
    /// List available releases
    #[command(alias("ls"))]
    List(ReleaseListArgs),

    /// Print out detailed release information
    #[command(alias("desc"))]
    Describe(ReleaseDescribeArgs),

    /// Install a specific release
    #[command(aliases(["i", "in"]))]
    Install(ReleaseInstallArgs),

    /// Uninstall a release
    #[command(aliases(["rm", "un"]))]
    Uninstall(ReleaseUninstallArgs),
}

#[derive(Debug, Args)]
pub struct ReleaseListArgs {
    #[arg(short, long = "output", value_enum, default_value_t = Default::default())]
    output_type: OutputType,
}

#[derive(Debug, Args)]
pub struct ReleaseDescribeArgs {
    #[arg(name = "RELEASE")]
    release: String,

    #[arg(short, long = "output", value_enum, default_value_t = Default::default())]
    output_type: OutputType,
}

#[derive(Debug, Args)]
pub struct ReleaseInstallArgs {
    /// Release to install
    #[arg(name = "RELEASE")]
    release: String,

    /// Whitelist of product operators to install
    #[arg(short, long = "include", group = "products")]
    included_products: Vec<String>,

    /// Blacklist of product operators to install
    #[arg(short, long = "exclude", group = "products")]
    excluded_products: Vec<String>,

    /// Namespace in the cluster used to deploy the operators
    #[arg(long, default_value = DEFAULT_OPERATOR_NAMESPACE, visible_aliases(["operator-ns"]))]
    pub operator_namespace: String,

    #[command(flatten)]
    local_cluster: CommonClusterArgs,
}

#[derive(Debug, Args)]
pub struct ReleaseUninstallArgs {
    /// Name of the release to uninstall
    #[arg(name = "RELEASE")]
    release: String,

    /// Namespace in the cluster used to deploy the operators
    #[arg(long, default_value = DEFAULT_OPERATOR_NAMESPACE, visible_aliases(["operator-ns"]))]
    pub operator_namespace: String,
}

#[derive(Debug, Snafu)]
pub enum CmdError {
    #[snafu(display("failed to serialize YAML output"))]
    SerializeYamlOutput { source: serde_yaml::Error },

    #[snafu(display("failed to serialize JSON output"))]
    SerializeJsonOutput { source: serde_json::Error },

    #[snafu(display("failed to parse path/url"))]
    PathOrUrlParse { source: PathOrUrlParseError },

    #[snafu(display("failed to build release list"))]
    BuildList { source: list::Error },

    #[snafu(display("failed to install release"))]
    ReleaseInstall { source: release::Error },

    #[snafu(display("failed to uninstall release"))]
    ReleaseUninstall { source: release::Error },

    #[snafu(display("cluster argument error"))]
    CommonClusterArgs { source: CommonClusterArgsError },

    #[snafu(display("failed to create Kubernetes client"))]
    KubeClientCreate { source: k8s::Error },

    #[snafu(display("failed to create namespace '{namespace}'"))]
    NamespaceCreate {
        source: namespace::Error,
        namespace: String,
    },
}

impl ReleaseArgs {
    pub async fn run(&self, cli: &Cli, cache: Cache) -> Result<String, CmdError> {
        debug!("Handle release args");

        let transfer_client = xfer::Client::new_with(cache);
        let files = cli.get_release_files().context(PathOrUrlParseSnafu)?;
        let release_list = release::ReleaseList::build(&files, &transfer_client)
            .await
            .context(BuildListSnafu)?;

        if release_list.is_empty() {
            return Ok("No releases".into());
        }

        match &self.subcommand {
            ReleaseCommands::List(args) => list_cmd(args, cli, release_list).await,
            ReleaseCommands::Describe(args) => describe_cmd(args, cli, release_list).await,
            ReleaseCommands::Install(args) => install_cmd(args, cli, release_list).await,
            ReleaseCommands::Uninstall(args) => uninstall_cmd(args, cli, release_list).await,
        }
    }
}

#[instrument(skip(cli, release_list))]
async fn list_cmd(
    args: &ReleaseListArgs,
    cli: &Cli,
    release_list: release::ReleaseList,
) -> Result<String, CmdError> {
    info!("Listing releases");

    match args.output_type {
        OutputType::Plain | OutputType::Table => {
            if release_list.is_empty() {
                return Ok("No releases".into());
            }

            let (arrangement, preset) = match args.output_type {
                OutputType::Plain => (ContentArrangement::Disabled, NOTHING),
                _ => (ContentArrangement::Dynamic, UTF8_FULL),
            };

            let mut table = Table::new();
            table
                .set_header(vec!["#", "RELEASE", "RELEASE DATE", "DESCRIPTION"])
                .set_content_arrangement(arrangement)
                .load_preset(preset);

            for (index, (release_name, release_spec)) in release_list.iter().enumerate() {
                table.add_row(vec![
                    (index + 1).to_string(),
                    release_name.to_string(),
                    release_spec.date.clone(),
                    release_spec.description.clone(),
                ]);
            }

            let mut result = cli.result();

            result
                .with_command_hint(
                    "stackablectl release describe [OPTIONS] <RELEASE>",
                    "display further information for the specified release",
                )
                .with_command_hint(
                    "stackablectl release install [OPTIONS] <RELEASE>",
                    "install a release",
                )
                .with_output(table.to_string());

            Ok(result.render())
        }
        OutputType::Json => serde_json::to_string(&release_list).context(SerializeJsonOutputSnafu),
        OutputType::Yaml => serde_yaml::to_string(&release_list).context(SerializeYamlOutputSnafu),
    }
}

#[instrument(skip(cli, release_list))]
async fn describe_cmd(
    args: &ReleaseDescribeArgs,
    cli: &Cli,
    release_list: release::ReleaseList,
) -> Result<String, CmdError> {
    info!("Describing release");

    let release = release_list.get(&args.release);

    match release {
        Some(release) => match args.output_type {
            OutputType::Plain | OutputType::Table => {
                let arrangement = match args.output_type {
                    OutputType::Plain => ContentArrangement::Disabled,
                    _ => ContentArrangement::Dynamic,
                };

                let mut product_table = Table::new();

                product_table
                    .set_header(vec!["PRODUCT", "OPERATOR VERSION"])
                    .set_content_arrangement(ContentArrangement::Dynamic)
                    .load_preset(NOTHING);

                for (product_name, product) in &release.products {
                    product_table.add_row(vec![product_name, &product.version.to_string()]);
                }

                let mut table = Table::new();

                table
                    .set_content_arrangement(arrangement)
                    .load_preset(NOTHING)
                    .add_row(vec!["RELEASE", &args.release])
                    .add_row(vec!["RELEASE DATE", release.date.as_str()])
                    .add_row(vec!["DESCRIPTION", release.description.as_str()])
                    .add_row(vec![
                        "INCLUDED PRODUCTS",
                        product_table.to_string().as_str(),
                    ]);

                let mut result = cli.result();

                result
                    .with_command_hint(
                        format!("stackablectl release install {}", args.release),
                        "install the release",
                    )
                    .with_command_hint("stackablectl release list", "list all available releases")
                    .with_output(table.to_string());

                Ok(result.render())
            }
            OutputType::Json => serde_json::to_string(&release).context(SerializeJsonOutputSnafu),
            OutputType::Yaml => serde_yaml::to_string(&release).context(SerializeYamlOutputSnafu),
        },
        None => Ok("No such release".into()),
    }
}

#[instrument(skip(cli, release_list))]
async fn install_cmd(
    args: &ReleaseInstallArgs,
    cli: &Cli,
    release_list: release::ReleaseList,
) -> Result<String, CmdError> {
    match release_list.get(&args.release) {
        Some(release) => {
            let mut output = cli.result();

            // Install local cluster if needed
            args.local_cluster
                .install_if_needed()
                .await
                .context(CommonClusterArgsSnafu)?;

            let client = Client::new().await.context(KubeClientCreateSnafu)?;

            // Create operator namespace if needed
            namespace::create_if_needed(&client, args.operator_namespace.clone())
                .await
                .context(NamespaceCreateSnafu {
                    namespace: args.operator_namespace.clone(),
                })?;

            release
                .install(
                    &args.included_products,
                    &args.excluded_products,
                    &args.operator_namespace,
                    &ChartSourceType::from(cli.chart_type()),
                )
                .await
                .context(ReleaseInstallSnafu)?;

            output
                .with_command_hint(
                    "stackablectl operator installed",
                    "list installed operators",
                )
                .with_output(format!("Installed release '{}'", args.release));

            Ok(output.render())
        }
        None => Ok("No such release".into()),
    }
}

#[instrument(skip(cli, release_list))]
async fn uninstall_cmd(
    args: &ReleaseUninstallArgs,
    cli: &Cli,
    release_list: release::ReleaseList,
) -> Result<String, CmdError> {
    match release_list.get(&args.release) {
        Some(release) => {
            release
                .uninstall(&args.operator_namespace)
                .context(ReleaseUninstallSnafu)?;

            let mut result = cli.result();

            result
                .with_command_hint("stackablectl release list", "list available releases")
                .with_output(format!("Uninstalled release '{}'", args.release));

            Ok(result.render())
        }
        None => Ok("No such release".into()),
    }
}
