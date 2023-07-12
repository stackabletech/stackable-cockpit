use clap::{Args, Subcommand};
use comfy_table::{presets::NOTHING, ContentArrangement, Table};
use snafu::{ResultExt, Snafu};
use tracing::{info, instrument};

use stackable::{
    common::ListError,
    platform::release::{ReleaseInstallError, ReleaseList, ReleaseSpec, ReleaseUninstallError},
    utils::path::PathOrUrlParseError,
};

use crate::{
    cli::{CacheSettingsError, Cli, CommonClusterArgs, CommonClusterArgsError, OutputType},
    output::{ResultOutput, TabledOutput},
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
    #[snafu(display("unable to format yaml output"), context(false))]
    YamlOutputFormatError { source: serde_yaml::Error },

    #[snafu(display("unable to format json output"), context(false))]
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

    #[snafu(display("no release with name '{name}'"))]
    NoSuchRelease { name: String },
}

impl ResultOutput for ReleaseList {
    const EMPTY_MESSAGE: &'static str = "No releases";
    type Error = ReleaseCmdError;
}

impl TabledOutput for ReleaseList {
    const COLUMNS: &'static [&'static str] = &["#", "RELEASE", "RELEASE DATE", "DESCRIPTION"];
    type Row = Vec<String>;

    fn rows(&self) -> Vec<Self::Row> {
        self.inner()
            .iter()
            .enumerate()
            .map(|(index, (release_name, release_spec))| {
                vec![
                    (index + 1).to_string(),
                    release_name.clone(),
                    release_spec.date.clone(),
                    release_spec.description.clone(),
                ]
            })
            .collect()
    }
}

impl ResultOutput for ReleaseSpec {
    type Error = ReleaseCmdError;
}

impl TabledOutput for ReleaseSpec {
    type Row = Vec<String>;

    fn rows(&self) -> Vec<Self::Row> {
        let mut stacklet_table = Table::new();

        stacklet_table
            .set_content_arrangement(ContentArrangement::Dynamic)
            .load_preset(NOTHING)
            .set_header(vec!["PRODUCT", "OPERATOR VERSION"]);

        for (product_name, product) in &self.products {
            stacklet_table.add_row(vec![product_name, &product.version]);
        }

        vec![
            vec!["RELEASE DATE".into(), self.date.clone()],
            vec!["DESCRIPTION".into(), self.description.clone()],
            vec!["INCLUDED STACKLETS".into(), stacklet_table.to_string()],
        ]
    }
}

impl ReleaseArgs {
    pub async fn run(&self, common_args: &Cli) -> Result<String, ReleaseCmdError> {
        let files = common_args
            .get_release_files()
            .context(PathOrUrlParseSnafu)?;

        let release_list = ReleaseList::build(&files, &common_args.cache_settings()?)
            .await
            .context(ListSnafu)?;

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

    Ok(release_list.output(args.output_type)?)
}

#[instrument]
async fn describe_cmd(
    args: &ReleaseDescribeArgs,
    release_list: ReleaseList,
) -> Result<String, ReleaseCmdError> {
    info!("Describing release");

    let release = release_list
        .get(&args.release)
        .ok_or(ReleaseCmdError::NoSuchRelease {
            name: args.release.clone(),
        })?;

    Ok(release.output(args.output_type)?)
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
