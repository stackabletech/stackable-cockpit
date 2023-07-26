use clap::{Args, Subcommand};
use comfy_table::{
    presets::{NOTHING, UTF8_FULL},
    ContentArrangement, Table,
};
use snafu::{ResultExt, Snafu};
use tracing::{debug, info, instrument};

use stackable_cockpit::{
    common::ListError,
    platform::release::{ReleaseInstallError, ReleaseList, ReleaseUninstallError},
    utils::path::PathOrUrlParseError,
    xfer::{FileTransferClient, FileTransferError},
};

use crate::cli::{CacheSettingsError, Cli, CommonClusterArgs, CommonClusterArgsError, OutputType};

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

    #[command(flatten)]
    local_cluster: CommonClusterArgs,
}

#[derive(Debug, Args)]
pub struct ReleaseUninstallArgs {
    /// Name of the release to uninstall
    #[arg(name = "RELEASE")]
    release: String,
}

#[derive(Debug, Snafu)]
pub enum ReleaseCmdError {
    #[snafu(display("unable to format yaml output"))]
    YamlOutputFormatError { source: serde_yaml::Error },

    #[snafu(display("unable to format json output"))]
    JsonOutputFormatError { source: serde_json::Error },

    #[snafu(display("path/url parse error"))]
    PathOrUrlParseError { source: PathOrUrlParseError },

    #[snafu(display("cache settings resolution error"), context(false))]
    CacheSettingsError { source: CacheSettingsError },

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
}

impl ReleaseArgs {
    pub async fn run(&self, common_args: &Cli) -> Result<String, ReleaseCmdError> {
        debug!("Handle release args");

        let transfer_client = FileTransferClient::new(common_args.cache_settings()?)
            .await
            .context(TransferSnafu)?;

        let files = common_args
            .get_release_files()
            .context(PathOrUrlParseSnafu)?;

        let release_list = ReleaseList::build(&files, &transfer_client)
            .await
            .context(ListSnafu)?;

        if release_list.inner().is_empty() {
            return Ok("No releases".into());
        }

        match &self.subcommand {
            ReleaseCommands::List(args) => list_cmd(args, release_list).await,
            ReleaseCommands::Describe(args) => describe_cmd(args, release_list).await,
            ReleaseCommands::Install(args) => install_cmd(args, common_args, release_list).await,
            ReleaseCommands::Uninstall(args) => {
                uninstall_cmd(args, common_args, release_list).await
            }
        }
    }
}

#[instrument]
async fn list_cmd(
    args: &ReleaseListArgs,
    release_list: ReleaseList,
) -> Result<String, ReleaseCmdError> {
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

            Ok(table.to_string())
        }
        OutputType::Json => serde_json::to_string(&release_list).context(JsonOutputFormatSnafu),
        OutputType::Yaml => serde_yaml::to_string(&release_list).context(YamlOutputFormatSnafu),
    }
}

#[instrument]
async fn describe_cmd(
    args: &ReleaseDescribeArgs,
    release_list: ReleaseList,
) -> Result<String, ReleaseCmdError> {
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
                    product_table.add_row(vec![product_name, &product.version]);
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

                Ok(table.to_string())
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
    common_args: &Cli,
    release_list: ReleaseList,
) -> Result<String, ReleaseCmdError> {
    info!("Installing release");

    // Install local cluster if needed
    args.local_cluster
        .install_if_needed(None)
        .await
        .context(CommonClusterArgsSnafu)?;

    match release_list.get(&args.release) {
        Some(release) => {
            release
                .install(
                    &args.included_products,
                    &args.excluded_products,
                    &common_args.operator_namespace,
                )
                .context(ReleaseInstallSnafu)?;

            Ok("Installed release".into())
        }
        None => Ok("No such release".into()),
    }
}

async fn uninstall_cmd(
    args: &ReleaseUninstallArgs,
    common_args: &Cli,
    release_list: ReleaseList,
) -> Result<String, ReleaseCmdError> {
    info!("Installing release");

    match release_list.get(&args.release) {
        Some(release) => {
            release
                .uninstall(&common_args.operator_namespace)
                .context(ReleaseUninstallSnafu)?;

            Ok("Installed release".into())
        }
        None => Ok("No such release".into()),
    }
}
