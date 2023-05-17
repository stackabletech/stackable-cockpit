// External crates
use clap::{Args, Subcommand};
use comfy_table::{
    presets::{NOTHING, UTF8_FULL},
    ContentArrangement, Table,
};
use snafu::{ResultExt, Snafu};
use tracing::{info, instrument};
use xdg::BaseDirectoriesError;

// Stackable Library
use stackable::{
    common::ListError,
    constants::DEFAULT_LOCAL_CLUSTER_NAME,
    platform::release::{ReleaseInstallError, ReleaseList, ReleaseUninstallError},
    utils::path::PathOrUrlParseError,
};

// Local
use crate::{
    cli::{Cli, ClusterType, OutputType},
    constants::CACHE_HOME_PATH,
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
    #[arg(name = "RELEASE")]
    release: String,

    /// Whitelist of product operators to install
    #[arg(short, long = "include", group = "products")]
    included_products: Vec<String>,

    /// Blacklist of product operators to install
    #[arg(short, long = "exclude", group = "products")]
    excluded_products: Vec<String>,

    /// Type of local cluster to use for testing
    #[arg(short = 'c', long = "cluster", value_enum, value_name = "CLUSTER_TYPE", default_value_t = ClusterType::default())]
    #[arg(
        long_help = "If specified, a local Kubernetes cluster consisting of 4 nodes (1 for
control-plane and 3 workers) will be created for testing purposes. Currently
'kind' and 'minikube' are supported. Both require a working Docker
installation on the system."
    )]
    cluster_type: ClusterType,

    /// Name of the local cluster
    #[arg(long, default_value = DEFAULT_LOCAL_CLUSTER_NAME)]
    cluster_name: String,

    /// Number of total nodes in the local cluster
    #[arg(long, default_value_t = 3)]
    #[arg(long_help = "Number of total nodes in the local cluster

This number specifies the total number of nodes, which combines control plane
and worker nodes. The number of control plane nodes can be customized with the
--cluster-cp-nodes argument. The default number of control plane nodes is '1'.
So when specifying a total number of nodes of '4', there will be one control
plane node and three worker nodes.")]
    cluster_nodes: usize,

    /// Number of control plane nodes in the local cluster
    #[arg(long, default_value_t = 1)]
    #[arg(long_help = "Number of control plane nodes in the local cluster

This number must be smaller than --cluster-nodes. If this is not the case,
stackablectl will silently fall back to the value '1'.")]
    cluster_cp_nodes: usize,
}

#[derive(Debug, Args)]
pub struct ReleaseUninstallArgs {
    /// Name of the release to uninstall
    #[arg(name = "RELEASE")]
    release: String,
}

#[derive(Debug, Snafu)]
pub enum ReleaseCmdError {
    #[snafu(display("unable to format yaml output:: {source}"))]
    YamlError { source: serde_yaml::Error },

    #[snafu(display("unable to format json output:: {source}"))]
    JsonError { source: serde_json::Error },

    #[snafu(display("path/url parse error: {source}"))]
    PathOrUrlParseError { source: PathOrUrlParseError },

    #[snafu(display("xdg base directory error: {source}"))]
    XdgError { source: BaseDirectoriesError },

    #[snafu(display("list error: {source}"))]
    ListError { source: ListError },

    #[snafu(display("release install error: {source}"))]
    ReleaseInstallError { source: ReleaseInstallError },

    #[snafu(display("release uninstall error: {source}"))]
    ReleaseUninstallError { source: ReleaseUninstallError },
}

impl ReleaseArgs {
    pub async fn run(&self, common_args: &Cli) -> Result<String, ReleaseCmdError> {
        match &self.subcommand {
            ReleaseCommands::List(args) => list_cmd(args, common_args).await,
            ReleaseCommands::Describe(args) => describe_cmd(args, common_args).await,
            ReleaseCommands::Install(args) => install_cmd(args, common_args).await,
            ReleaseCommands::Uninstall(args) => uninstall_cmd(args, common_args).await,
        }
    }
}

#[instrument]
async fn list_cmd(args: &ReleaseListArgs, common_args: &Cli) -> Result<String, ReleaseCmdError> {
    info!("Listing releases");

    let files = common_args
        .get_release_files()
        .context(PathOrUrlParseSnafu {})?;

    let cache_home_path = xdg::BaseDirectories::with_prefix(CACHE_HOME_PATH)
        .context(XdgSnafu {})?
        .get_cache_home();

    let release_list = ReleaseList::build(files, (cache_home_path, !common_args.no_cache).into())
        .await
        .context(ListSnafu {})?;

    match args.output_type {
        OutputType::Plain => {
            if release_list.inner().is_empty() {
                return Ok("No releases".into());
            }

            let mut table = Table::new();

            table
                .set_content_arrangement(ContentArrangement::Dynamic)
                .load_preset(UTF8_FULL)
                .set_header(vec!["RELEASE", "RELEASE DATE", "DESCRIPTION"]);

            for (release_name, release_spec) in release_list.inner() {
                table.add_row(vec![
                    release_name.to_string(),
                    release_spec.date.clone(),
                    release_spec.description.clone(),
                ]);
            }

            Ok(table.to_string())
        }
        OutputType::Json => serde_json::to_string(&release_list).context(JsonSnafu {}),
        OutputType::Yaml => serde_yaml::to_string(&release_list).context(YamlSnafu {}),
    }
}

#[instrument]
async fn describe_cmd(
    args: &ReleaseDescribeArgs,
    common_args: &Cli,
) -> Result<String, ReleaseCmdError> {
    info!("Describing release");

    let files = common_args
        .get_release_files()
        .context(PathOrUrlParseSnafu {})?;

    let cache_home_path = xdg::BaseDirectories::with_prefix(CACHE_HOME_PATH)
        .context(XdgSnafu {})?
        .get_cache_home();

    let release_list = ReleaseList::build(files, (cache_home_path, !common_args.no_cache).into())
        .await
        .context(ListSnafu {})?;

    if release_list.inner().is_empty() {
        return Ok("No releases".into());
    }

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
            OutputType::Json => serde_json::to_string(&release).context(JsonSnafu {}),
            OutputType::Yaml => serde_yaml::to_string(&release).context(YamlSnafu {}),
        },
        None => Ok("No such release".into()),
    }
}

#[instrument]
async fn install_cmd(
    args: &ReleaseInstallArgs,
    common_args: &Cli,
) -> Result<String, ReleaseCmdError> {
    info!("Installing release");

    let files = common_args
        .get_release_files()
        .context(PathOrUrlParseSnafu {})?;

    let cache_home_path = xdg::BaseDirectories::with_prefix(CACHE_HOME_PATH)
        .context(XdgSnafu {})?
        .get_cache_home();

    let release_list = ReleaseList::build(files, (cache_home_path, !common_args.no_cache).into())
        .await
        .context(ListSnafu {})?;

    if release_list.inner().is_empty() {
        return Ok("No releases".into());
    }

    match release_list.get(&args.release) {
        Some(release) => {
            release
                .install(
                    &args.included_products,
                    &args.excluded_products,
                    &common_args.operator_namespace,
                )
                .context(ReleaseInstallSnafu {})?;

            Ok("Installed release".into())
        }
        None => Ok("No such release".into()),
    }
}

async fn uninstall_cmd(
    args: &ReleaseUninstallArgs,
    common_args: &Cli,
) -> Result<String, ReleaseCmdError> {
    info!("Installing release");

    let files = common_args
        .get_release_files()
        .context(PathOrUrlParseSnafu {})?;

    let cache_home_path = xdg::BaseDirectories::with_prefix(CACHE_HOME_PATH)
        .context(XdgSnafu {})?
        .get_cache_home();

    let release_list = ReleaseList::build(files, (cache_home_path, !common_args.no_cache).into())
        .await
        .context(ListSnafu {})?;

    if release_list.inner().is_empty() {
        return Ok("No releases".into());
    }

    match release_list.get(&args.release) {
        Some(release) => {
            release
                .uninstall(&common_args.operator_namespace)
                .context(ReleaseUninstallSnafu {})?;

            Ok("Installed release".into())
        }
        None => Ok("No such release".into()),
    }
}
