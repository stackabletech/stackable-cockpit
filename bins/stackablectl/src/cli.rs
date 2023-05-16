use std::env;

use clap::{Parser, Subcommand, ValueEnum, ValueHint};
use stackable::{
    constants::{
        DEFAULT_NAMESPACE, HELM_REPO_NAME_DEV, HELM_REPO_NAME_STABLE, HELM_REPO_NAME_TEST,
    },
    helm::{self, HelmError},
    utils::path::{
        IntoPathOrUrl, IntoPathsOrUrls, ParsePathsOrUrls, PathOrUrl, PathOrUrlParseError,
    },
};
use tracing::{debug, instrument, Level};

use crate::{
    cmds::{
        cache::CacheArgs, completions::CompletionsArgs, demo::DemoArgs, operator::OperatorArgs,
        release::ReleaseArgs, services::ServicesArgs, stack::StackArgs,
    },
    constants::{
        ENV_KEY_DEMO_FILES, ENV_KEY_RELEASE_FILES, ENV_KEY_STACK_FILES, HELM_REPO_URL_DEV,
        HELM_REPO_URL_STABLE, HELM_REPO_URL_TEST, REMOTE_DEMO_FILE, REMOTE_RELEASE_FILE,
        REMOTE_STACK_FILE,
    },
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
    #[arg(short, long, default_value = DEFAULT_NAMESPACE)]
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
    pub release_files: Vec<String>,

    /// Provide a custom Helm stable repository URL
    #[arg(long, value_name = "URL", value_hint = ValueHint::Url, default_value = HELM_REPO_URL_STABLE)]
    pub helm_repo_stable: String,

    /// Provide a custom Helm test repository URL
    #[arg(long, value_name = "URL", value_hint = ValueHint::Url, default_value = HELM_REPO_URL_TEST)]
    pub helm_repo_test: String,

    /// Provide a custom Helm dev repository URL
    #[arg(long, value_name = "URL", value_hint = ValueHint::Url, default_value = HELM_REPO_URL_DEV)]
    pub helm_repo_dev: String,

    #[command(subcommand)]
    pub subcommand: Commands,
}

impl Cli {
    /// Returns a list of demo files, consisting of entries which are either a path or URL. The list of files combines
    /// the default demo file URL, [`REMOTE_DEMO_FILE`], files provided by the ENV variable [`ENV_KEY_DEMO_FILES`], and
    /// lastly, files provided by the CLI argument `--demo-file`.
    pub fn get_demo_files(&self) -> Result<Vec<PathOrUrl>, PathOrUrlParseError> {
        let mut files: Vec<PathOrUrl> = vec![REMOTE_DEMO_FILE.into_path_or_url()?];

        let env_files = match env::var(ENV_KEY_DEMO_FILES) {
            Ok(env_files) => env_files.parse_paths_or_urls()?,
            Err(_) => vec![],
        };
        files.extend(env_files);

        let arg_files = self.demo_files.clone().into_paths_or_urls()?;
        files.extend(arg_files);

        Ok(files)
    }

    /// Returns a list of stack files, consisting of entries which are either a path or URL. The list of files combines
    /// the default stack file URL, [`REMOTE_STACK_FILE`], files provided by the ENV variable [`ENV_KEY_STACK_FILES`],
    /// and lastly, files provided by the CLI argument `--stack-file`.
    pub fn get_stack_files(&self) -> Result<Vec<PathOrUrl>, PathOrUrlParseError> {
        let mut files: Vec<PathOrUrl> = vec![REMOTE_STACK_FILE.into_path_or_url()?];

        let env_files = match env::var(ENV_KEY_STACK_FILES) {
            Ok(env_files) => env_files.parse_paths_or_urls()?,
            Err(_) => vec![],
        };
        files.extend(env_files);

        let arg_files = self.stack_files.clone().into_paths_or_urls()?;
        files.extend(arg_files);

        Ok(files)
    }

    /// Returns a list of release files, consisting of entries which are either a path or URL. The list of files
    /// combines the default demo file URL, [`REMOTE_RELEASE_FILE`], files provided by the ENV variable
    /// [`ENV_KEY_RELEASE_FILES`], and lastly, files provided by the CLI argument `--release-file`.
    pub fn get_release_files(&self) -> Result<Vec<PathOrUrl>, PathOrUrlParseError> {
        let mut files: Vec<PathOrUrl> = vec![REMOTE_RELEASE_FILE.into_path_or_url()?];

        let env_files = match env::var(ENV_KEY_RELEASE_FILES) {
            Ok(env_files) => env_files.parse_paths_or_urls()?,
            Err(_) => vec![],
        };
        files.extend(env_files);

        let arg_files = self.release_files.clone().into_paths_or_urls()?;
        files.extend(arg_files);

        Ok(files)
    }

    /// Adds the default (or custom) Helm repository URLs. Internally this calls the Helm SDK written in Go through the
    /// `go-helm-wrapper`.
    #[instrument]
    pub fn add_helm_repos(&self) -> Result<(), HelmError> {
        debug!("Add Helm repos");

        // Stable repository
        helm::add_repo(HELM_REPO_NAME_STABLE, &self.helm_repo_stable)?;

        // Test repository
        helm::add_repo(HELM_REPO_NAME_TEST, &self.helm_repo_test)?;

        // Dev repository
        helm::add_repo(HELM_REPO_NAME_DEV, &self.helm_repo_dev)?;

        Ok(())
    }
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

#[derive(Clone, Debug, Default, ValueEnum)]
pub enum OutputType {
    /// Print output formatted as plain text
    #[default]
    Plain,

    /// Print output formatted as JSON
    Json,

    /// Print output formatted as YAML
    Yaml,
}

#[derive(Clone, Debug, Default, ValueEnum)]
pub enum ClusterType {
    /// Use a kind cluster, see 'https://docs.stackable.tech/home/getting_started.html#_installing_kubernetes_using_kind'
    #[default]
    Kind,

    /// Use a minikube cluster (CURRENTLY UNSUPPORTED)
    Minikube,
}
