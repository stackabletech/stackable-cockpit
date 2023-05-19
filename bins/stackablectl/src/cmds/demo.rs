// External crates
use clap::{Args, Subcommand};
use comfy_table::{
    presets::{NOTHING, UTF8_FULL},
    ContentArrangement, Row, Table,
};
use snafu::{ResultExt, Snafu};
use xdg::BaseDirectoriesError;

// Stackable library
use stackable::{
    cluster::ClusterError,
    common::ListError,
    platform::{
        demo::DemoList,
        release::ReleaseList,
        stack::{StackError, StackList},
    },
    utils::{params::IntoParametersError, path::PathOrUrlParseError, read::CacheSettings},
};

// Local
use crate::{
    cli::{Cli, CommonClusterArgs, OutputType},
    constants::CACHE_HOME_PATH,
};

#[derive(Debug, Args)]
pub struct DemoArgs {
    #[command(subcommand)]
    subcommand: DemoCommands,
}

#[derive(Debug, Subcommand)]
pub enum DemoCommands {
    /// List available demos
    #[command(alias("ls"))]
    List(DemoListArgs),

    /// Print out detailed demo information
    #[command(alias("desc"))]
    Describe(DemoDescribeArgs),

    /// Install a specific demo
    #[command(aliases(["i", "in"]))]
    Install(DemoInstallArgs),

    #[command(aliases(["rm", "un"]))]
    Uninstall(DemoUninstallArgs),
}

#[derive(Debug, Args)]
pub struct DemoListArgs {
    #[arg(short, long = "output", value_enum, default_value_t = Default::default())]
    output_type: OutputType,
}

#[derive(Debug, Args)]
pub struct DemoDescribeArgs {
    /// Demo to describe
    #[arg(name = "DEMO")]
    demo_name: String,

    #[arg(short, long = "output", value_enum, default_value_t = Default::default())]
    output_type: OutputType,
}

#[derive(Debug, Args)]
pub struct DemoInstallArgs {
    /// Demo to install
    #[arg(name = "DEMO")]
    demo_name: String,

    /// List of parameters to use when installing the stack
    #[arg(short, long)]
    stack_parameters: Vec<String>,

    /// List of parameters to use when installing the demo
    #[arg(short, long)]
    parameters: Vec<String>,

    #[command(flatten)]
    local_cluster: CommonClusterArgs,
}

#[derive(Debug, Args)]
pub struct DemoUninstallArgs {}

#[derive(Debug, Snafu)]
pub enum DemoCmdError {
    #[snafu(display("io error: {source}"))]
    IoError { source: std::io::Error },

    #[snafu(display("yaml error: {source}"))]
    YamlError { source: serde_yaml::Error },

    #[snafu(display("json error: {source}"))]
    JsonError { source: serde_json::Error },

    #[snafu(display("no demo with name '{name}'"))]
    NoSuchDemo { name: String },

    #[snafu(display("no stack with name '{name}'"))]
    NoSuchStack { name: String },

    #[snafu(display("failed to convert input parameters to validated parameters: {source}"))]
    IntoParametersError { source: IntoParametersError },

    #[snafu(display("list error: {source}"))]
    ListError { source: ListError },

    #[snafu(display("stack error: {source}"))]
    StackError { source: StackError },

    #[snafu(display("path/url parse error: {source}"))]
    PathOrUrlParseError { source: PathOrUrlParseError },

    #[snafu(display("xdg base directory error: {source}"))]
    XdgError { source: BaseDirectoriesError },

    #[snafu(display("cluster error"))]
    ClusterError { source: ClusterError },
}

impl DemoArgs {
    pub async fn run(&self, common_args: &Cli) -> Result<String, DemoCmdError> {
        // Build demo list based on the (default) remote demo file, and additional files provided by the
        // STACKABLE_DEMO_FILES env variable or the --demo-files CLI argument.
        let files = common_args
            .get_demo_files()
            .context(PathOrUrlParseSnafu {})?;

        let cache_file_path = xdg::BaseDirectories::with_prefix(CACHE_HOME_PATH)
            .context(XdgSnafu {})?
            .get_cache_home();

        let cache_settings = CacheSettings::from((cache_file_path, !common_args.no_cache));
        let list = DemoList::build(&files, cache_settings)
            .await
            .context(ListSnafu {})?;

        match &self.subcommand {
            DemoCommands::List(args) => list_cmd(args, list).await,
            DemoCommands::Describe(args) => describe_cmd(args, list).await,
            DemoCommands::Install(args) => install_cmd(args, common_args, list).await,
            DemoCommands::Uninstall(args) => uninstall_cmd(args, list),
        }
    }
}

/// Print out a list of demos, either as a table (plain), JSON or YAML
async fn list_cmd(args: &DemoListArgs, list: DemoList) -> Result<String, DemoCmdError> {
    match args.output_type {
        OutputType::Plain => {
            let mut table = Table::new();

            table
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_header(vec!["NAME", "STACK", "DESCRIPTION"])
                .load_preset(UTF8_FULL);

            for (demo_name, demo_spec) in list.inner() {
                let row = Row::from(vec![
                    demo_name.clone(),
                    demo_spec.stack.clone(),
                    demo_spec.description.clone(),
                ]);
                table.add_row(row);
            }

            Ok(table.to_string())
        }
        OutputType::Json => Ok(serde_json::to_string(&list.inner()).context(JsonSnafu {})?),
        OutputType::Yaml => Ok(serde_yaml::to_string(&list.inner()).context(YamlSnafu {})?),
    }
}

/// Describe a specific demo by printing out a table (plain), JSON or YAML
async fn describe_cmd(args: &DemoDescribeArgs, list: DemoList) -> Result<String, DemoCmdError> {
    let demo = list.get(&args.demo_name).ok_or(DemoCmdError::NoSuchDemo {
        name: args.demo_name.clone(),
    })?;

    match args.output_type {
        OutputType::Plain => {
            let mut table = Table::new();
            table
                .load_preset(NOTHING)
                .set_content_arrangement(ContentArrangement::Dynamic)
                .add_row(vec!["DEMO", &args.demo_name])
                .add_row(vec!["DESCRIPTION", &demo.description]);

            if let Some(documentation) = &demo.documentation {
                table.add_row(vec!["DOCUMENTATION", documentation]);
            }

            table
                .add_row(vec!["STACK", &demo.stack])
                .add_row(vec!["LABELS", &demo.labels.join(", ")]);

            // TODO (Techassi): Add parameter output

            Ok(table.to_string())
        }
        OutputType::Json => Ok(serde_json::to_string(&demo).context(JsonSnafu {})?),
        OutputType::Yaml => Ok(serde_yaml::to_string(&demo).context(YamlSnafu {})?),
    }
}

/// Install a specific demo
async fn install_cmd(
    args: &DemoInstallArgs,
    common_args: &Cli,
    list: DemoList,
) -> Result<String, DemoCmdError> {
    // Get the demo spec by name from the list
    let demo_spec = list.get(&args.demo_name).ok_or(DemoCmdError::NoSuchDemo {
        name: args.demo_name.clone(),
    })?;

    // Build demo list based on the (default) remote demo file, and additional files provided by the
    // STACKABLE_DEMO_FILES env variable or the --demo-files CLI argument.
    let files = common_args
        .get_stack_files()
        .context(PathOrUrlParseSnafu {})?;

    let cache_home_path = xdg::BaseDirectories::with_prefix(CACHE_HOME_PATH)
        .context(XdgSnafu {})?
        .get_cache_home();

    let stack_list = StackList::build(&files, (cache_home_path, !common_args.no_cache).into())
        .await
        .context(ListSnafu {})?;

    // Get the stack spec based on the name defined in the demo spec
    let stack_spec = stack_list
        .get(&demo_spec.stack)
        .ok_or(DemoCmdError::NoSuchStack {
            name: demo_spec.stack.clone(),
        })?;

    // TODO (Techassi): Try to move all this boilerplate code to build the lists out of here
    let files = common_args
        .get_stack_files()
        .context(PathOrUrlParseSnafu {})?;

    let cache_home_path = xdg::BaseDirectories::with_prefix(CACHE_HOME_PATH)
        .context(XdgSnafu {})?
        .get_cache_home();

    let release_list = ReleaseList::build(&files, (cache_home_path, !common_args.no_cache).into())
        .await
        .context(ListSnafu {})?;

    // Install local cluster if needed
    args.local_cluster
        .install_if_needed(None, None)
        .await
        .context(ClusterSnafu {})?;

    // Install the stack
    stack_spec
        .install(release_list, &common_args.operator_namespace)
        .context(StackSnafu {})?;

    // Install stack manifests
    stack_spec
        .install_stack_manifests(&args.stack_parameters, &common_args.operator_namespace)
        .await
        .context(StackSnafu {})?;

    // Install demo manifests
    stack_spec
        .install_demo_manifests(
            &demo_spec.manifests,
            &demo_spec.parameters,
            &args.parameters,
            &common_args.operator_namespace,
        )
        .await
        .context(StackSnafu {})?;

    Ok("".into())
}

fn uninstall_cmd(_args: &DemoUninstallArgs, _list: DemoList) -> Result<String, DemoCmdError> {
    todo!()
}
