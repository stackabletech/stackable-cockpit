use clap::{Parser, Subcommand, ValueEnum, ValueHint};
use stackable::constants::DEFAULT_STACKABLE_NAMESPACE;
use tracing::Level;

use crate::cmds::{
    cache::CacheArgs, completions::CompletionsArgs, demo::DemoArgs, operator::OperatorArgs,
    release::ReleaseArgs, services::ServicesArgs, stack::StackArgs,
};

#[derive(Debug, Parser)]
#[command(author, version, about, propagate_version = true)]
pub struct Cli {
    /// Log level this application uses
    #[arg(short, long)]
    pub log_level: Option<Level>,

    /// Do not cache the remote (default) demo, stack and release files
    #[arg(long)]
    #[arg(
        long_help = "Do not cache the remote (default) demo, stack and release files

Cached files are saved at '$XDG_CACHE_HOME/stackablectl', which is usually
'$HOME/.cache/stackablectl' when not explicitly set."
    )]
    pub no_cache: bool,

    /// Do not request any remote files via the network
    #[arg(long)]
    pub offline: bool,

    /// Namespace in the cluster used to deploy the products and operators
    #[arg(short, long, default_value = DEFAULT_STACKABLE_NAMESPACE)]
    pub namespace: String,

    /// Provide one or more additional (custom) demo file(s)
    #[arg(short, long = "demo-file", value_hint = ValueHint::FilePath)]
    #[arg(long_help = "Provide one or more additional (custom) demo file(s)

Demos are loaded in the following order: Remote (default) demo file, custom
demo files provided via the 'STACKABLE_DEMO_FILES' environment variable, and
lastly demo files provided via the '-d/--demo-file' argument(s). If there are
demos with the same name, the last demo definition will be used.

Use \"stackablectl -d path/to/demos1.yaml -d path/to/demos2.yaml [OPTIONS] <COMMAND>\"
to provide multiple additional demo files.")]
    pub demo_files: Vec<String>,

    /// Provide one or more additional (custom) stack file(s)
    #[arg(short, long = "stack-file", value_hint = ValueHint::FilePath)]
    #[arg(long_help = "Provide one or more additional (custom) stack file(s)
    
Stacks are loaded in the following order: Remote (default) stack file, custom
stack files provided via the 'STACKABLE_STACK_FILES' environment variable, and
lastly demo files provided via the '-s/--stack-file' argument(s). If there are
stacks with the same name, the last stack definition will be used.

Use \"stackablectl -s path/to/stacks1.yaml -s path/to/stacks2.yaml [OPTIONS] <COMMAND>\"
to provide multiple additional stack files.")]
    pub stack_files: Vec<String>,

    /// Provide one or more additional (custom) release file(s)
    #[arg(short, long = "release-file", value_hint = ValueHint::FilePath)]
    #[arg(long_help = "Provide one or more additional (custom) release file(s)
    
Releases are loaded in the following order: Remote (default) release file,
custom release files provided via the 'STACKABLE_RELEASE_FILES' environment
variable, and lastly release files provided via the '-r/--release-file'
argument(s). If there are releases with the same name, the last stack definition
will be used.

Use \"stackablectl -r path/to/realeases1.yaml -r path/to/realeases2.yaml [OPTIONS] <COMMAND>\"
to provide multiple additional stack files.")]
    pub release_file: Vec<String>,

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

    /// Interact with locally cached files
    Cache(CacheArgs),
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
