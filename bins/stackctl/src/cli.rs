use clap::{Parser, Subcommand, ValueEnum, ValueHint};
use stackable::constants::DEFAULT_STACKABLE_NAMESPACE;

use crate::cmds::{
    completions::CompletionsArgs, demo::DemoArgs, operator::OperatorArgs, release::ReleaseArgs,
    services::ServicesArgs, stack::StackArgs,
};

#[derive(Debug, Parser)]
#[command(author, version, about, propagate_version = true)]
pub struct Cli {
    /// Log level this application uses
    #[arg(short, long, value_enum, default_value_t = Default::default())]
    pub log_level: LogLevel,

    /// Namespace in the cluster used to deploy the products and operators
    #[arg(short, long, default_value = DEFAULT_STACKABLE_NAMESPACE)]
    pub namespace: String,

    /// Provide one or more additional (custom) demo file(s)
    #[arg(short, long = "additional-demo-file", value_hint = ValueHint::FilePath)]
    #[arg(long_help = "Provide one or more additional (custom) demo file(s)

Demos are loaded in the following order: Remote (default) demo file, custom
demo files provided via the 'STACKABLE_ADDITIONAL_DEMO_FILES' environment
variable, and lastly demo files provided via the '-a/--additional-demo-file'
argument(s). If there are demos with the same name, the later demo definition
will be used.

Use \"stackablectl -a path/to/demos1.yaml -a path/to/demos2.yaml [OPTIONS] <COMMAND>\"
to provide multiple additional demo files.")]
    pub additional_demo_files: Vec<String>,

    #[command(subcommand)]
    pub subcommand: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Interact with single operator instead of the full platform
    #[command(alias("op"))]
    Operator(OperatorArgs),

    /// Interact with all operators of the platform which are released together
    #[command(alias("re"))]
    Release(ReleaseArgs),

    /// Interact with stacks, which are ready-to-use product combinations
    #[command(alias("st"))]
    Stack(StackArgs),

    /// Interact with deployed services of products
    #[command(alias("svc"))]
    Services(ServicesArgs),

    /// Interact with demos, which are end-to-end usage demonstrations of the Stackable data platform
    Demo(DemoArgs),

    /// Generate shell completions for this tool
    #[command(alias("comp"))]
    Completions(CompletionsArgs),
}

#[derive(Clone, Debug, ValueEnum)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl Default for LogLevel {
    fn default() -> Self {
        Self::Warn
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum OutputType {
    /// Print output formatted as plain text
    Plain,

    /// Print output formatted as JSON
    Json,

    /// Print output formatted as YAML
    Yaml,
}

impl Default for OutputType {
    fn default() -> Self {
        Self::Plain
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum ClusterType {
    /// Don't use any local cluster
    None,

    /// Use a kind cluster, see 'https://docs.stackable.tech/home/getting_started.html#_installing_kubernetes_using_kind'
    Kind,

    /// Use a minikube cluster (CURRENTLY UNSUPPORTED)
    Minikube,
}

impl Default for ClusterType {
    fn default() -> Self {
        Self::None
    }
}
