use clap::{Args, Subcommand};
use comfy_table::{
    presets::{NOTHING, UTF8_FULL},
    ContentArrangement, Table,
};
use snafu::{ResultExt, Snafu};
use tracing::{debug, info, instrument};

use stackable_cockpit::{
    common::ListError,
    constants::DEFAULT_OPERATOR_NAMESPACE,
    platform::{
        namespace::{self, NamespaceError},
        release::{ReleaseInstallError, ReleaseList, ReleaseUninstallError},
    },
    utils::path::PathOrUrlParseError,
    xfer::{cache::Cache, FileTransferClient, FileTransferError},
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
    #[snafu(display("unable to format YAML output"))]
    YamlOutputFormatError { source: serde_yaml::Error },

    #[snafu(display("unable to format JSON output"))]
    JsonOutputFormatError { source: serde_json::Error },

    #[snafu(display("path/url parse error"))]
    PathOrUrlParseError { source: PathOrUrlParseError },

    #[snafu(display("list error"))]
    ListError { source: ListError },

    #[snafu(display("release install error"))]
    ReleaseInstallError { source: ReleaseInstallError },

    #[snafu(display("release uninstall error"))]
    ReleaseUninstallError { source: ReleaseUninstallError },

    #[snafu(display("cluster argument error"))]
    CommonClusterArgsError { source: CommonClusterArgsError },

    #[snafu(display("transfer error"))]
    TransferError { source: FileTransferError },

    #[snafu(display("failed to create namespace '{namespace}'"))]
    NamespaceError {
        source: NamespaceError,
        namespace: String,
    },
}

impl ReleaseArgs {
    pub async fn run(&self, cli: &Cli, cache: Cache) -> Result<String, CmdError> {
        debug!("Handle release args");

        let transfer_client = FileTransferClient::new_with(cache);

        let files = cli.get_release_files().context(PathOrUrlParseSnafu)?;

        let release_list = ReleaseList::build(&files, &transfer_client)
            .await
            .context(ListSnafu)?;

        if release_list.inner().is_empty() {
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

#[instrument]
async fn list_cmd(
    args: &ReleaseListArgs,
    cli: &Cli,
    release_list: ReleaseList,
) -> Result<String, CmdError> {
    info!("Listing releases");

    match args.output_type {
        OutputType::Plain => {
            if release_list.inner().is_empty() {
                return Ok("No releases".into());
            }

            let mut table = Table::new();

            table
                .set_content_arrangement(ContentArrangement::Dynamic)
                .load_preset(UTF8_FULL)
                .set_header(vec!["#", "RELEASE", "RELEASE DATE", "DESCRIPTION"]);

            for (index, (release_name, release_spec)) in release_list.inner().iter().enumerate() {
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

            // TODO (Techassi): Remove unwrap
            Ok(result.render().unwrap())
        }
        OutputType::Json => serde_json::to_string(&release_list).context(JsonOutputFormatSnafu),
        OutputType::Yaml => serde_yaml::to_string(&release_list).context(YamlOutputFormatSnafu),
    }
}

#[instrument]
async fn describe_cmd(
    args: &ReleaseDescribeArgs,
    cli: &Cli,
    release_list: ReleaseList,
) -> Result<String, CmdError> {
    info!("Describing release");

    let release = release_list.get(&args.release);

    match release {
        Some(release) => match args.output_type {
            OutputType::Plain => {
                let mut product_table = Table::new();

                product_table
                    .set_content_arrangement(ContentArrangement::Dynamic)
                    .load_preset(NOTHING)
                    .set_header(vec!["PRODUCT", "OPERATOR VERSION"]);

                for (product_name, product) in &release.products {
                    product_table.add_row(vec![product_name, &product.version.to_string()]);
                }

                let mut table = Table::new();

                table
                    .set_content_arrangement(ContentArrangement::Dynamic)
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
                        "install the demo",
                    )
                    .with_command_hint("stackablectl release list", "list all available releases")
                    .with_output(table.to_string());

                // TODO (Techassi): Remove unwrap
                Ok(result.render().unwrap())
            }
            OutputType::Json => serde_json::to_string(&release).context(JsonOutputFormatSnafu),
            OutputType::Yaml => serde_yaml::to_string(&release).context(YamlOutputFormatSnafu),
        },
        None => Ok("No such release".into()),
    }
}

#[instrument]
async fn install_cmd(
    args: &ReleaseInstallArgs,
    cli: &Cli,
    release_list: ReleaseList,
) -> Result<String, CmdError> {
    info!("Installing release");

    match release_list.get(&args.release) {
        Some(release) => {
            let mut output = cli.result();
            output.enable_progress(format!("Installing release '{}'", args.release));

            // Install local cluster if needed
            output.set_progress_message("Installing local cluster");
            args.local_cluster
                .install_if_needed(None)
                .await
                .context(CommonClusterArgsSnafu)?;

            // Create operator namespace if needed
            output.set_progress_message("Creating operator namespace");
            namespace::create_if_needed(args.operator_namespace.clone())
                .await
                .context(NamespaceSnafu {
                    namespace: args.operator_namespace.clone(),
                })?;

            output.set_progress_message("Installing release manifests");
            release
                .install(
                    &args.included_products,
                    &args.excluded_products,
                    &args.operator_namespace,
                )
                .context(ReleaseInstallSnafu)?;

            output
                .with_command_hint(
                    "stackablectl operator installed",
                    "list installed operators",
                )
                .with_output(format!("Installed release '{}'", args.release));

            output.finish_progress("Done");
            // TODO (Techassi): Remove unwrap
            Ok(output.render().unwrap())
        }
        None => Ok("No such release".into()),
    }
}

async fn uninstall_cmd(
    args: &ReleaseUninstallArgs,
    cli: &Cli,
    release_list: ReleaseList,
) -> Result<String, CmdError> {
    info!("Installing release");

    match release_list.get(&args.release) {
        Some(release) => {
            release
                .uninstall(&args.operator_namespace)
                .context(ReleaseUninstallSnafu)?;

            let mut result = cli.result();

            result
                .with_command_hint("stackablectl release list", "list available releases")
                .with_output(format!("Uninstalled release '{}'", args.release));

            // TODO (Techassi): Remove unwrap
            Ok(result.render().unwrap())
        }
        None => Ok("No such release".into()),
    }
}
