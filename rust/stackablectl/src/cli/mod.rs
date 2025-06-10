use std::env;

use clap::{Parser, Subcommand, ValueEnum};
use directories::ProjectDirs;
use snafu::{ResultExt, Snafu};
use stackable_cockpit::{
    constants::{HELM_REPO_NAME_DEV, HELM_REPO_NAME_STABLE, HELM_REPO_NAME_TEST},
    helm,
    platform::operator::ChartSourceType,
    utils::path::{
        IntoPathOrUrl, IntoPathsOrUrls, ParsePathsOrUrls, PathOrUrl, PathOrUrlParseError,
    },
    xfer::cache::Settings,
};
use tracing::{Level, instrument};

use crate::{
    args::{CommonFileArgs, CommonRepoArgs},
    cmds::{cache, completions, debug, demo, operator, release, stack, stacklet},
    constants::{
        DEMOS_REPOSITORY_DEMOS_SUBPATH, DEMOS_REPOSITORY_STACKS_SUBPATH, DEMOS_REPOSITORY_URL_BASE,
        ENV_KEY_DEMO_FILES, ENV_KEY_RELEASE_FILES, ENV_KEY_STACK_FILES, REMOTE_RELEASE_FILE,
        USER_DIR_APPLICATION_NAME, USER_DIR_ORGANIZATION_NAME, USER_DIR_QUALIFIER,
    },
    output::{ErrorContext, Output, ResultContext},
};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("failed to execute operator (sub)command"))]
    Operator { source: operator::CmdError },

    #[snafu(display("failed to execute release (sub)command"))]
    Release { source: release::CmdError },

    #[snafu(display("failed to execute stack (sub)command"))]
    Stack { source: stack::CmdError },

    #[snafu(display("failed to execute stacklet (sub)command"))]
    Stacklet { source: stacklet::CmdError },

    #[snafu(display("demo command error"))]
    Demo { source: demo::CmdError },

    #[snafu(display("completions command error"))]
    Completions { source: completions::CmdError },

    #[snafu(display("cache command error"))]
    Cache { source: cache::CmdError },

    #[snafu(display("debug command error"))]
    Debug { source: debug::CmdError },

    #[snafu(display("helm error"))]
    Helm { source: helm::Error },
}

#[derive(Debug, Parser)]
#[command(author, version, about, propagate_version = true)]
pub struct Cli {
    /// Log level this application uses
    #[arg(short, long, global = true)]
    pub log_level: Option<Level>,

    /// Do not cache the remote (default) demo, stack and release files
    #[arg(
        long,
        global = true,
        long_help = "Do not cache the remote (default) demo, stack and release files

Cached files are saved at '$XDG_CACHE_HOME/stackablectl', which is usually
'$HOME/.cache/stackablectl' when not explicitly set."
    )]
    pub no_cache: bool,

    #[command(flatten)]
    pub files: CommonFileArgs,

    #[command(flatten)]
    pub repos: CommonRepoArgs,

    #[command(subcommand)]
    pub subcommand: Commands,
}

impl Cli {
    /// Returns a list of demo files, consisting of entries which are either a path or URL. The list of files combines
    /// the default demo file URL constructed from [`DEMOS_REPOSITORY_URL_BASE`] and the provided branch, files provided
    /// by the ENV variable [`ENV_KEY_DEMO_FILES`], and lastly, files provided by the CLI argument `--demo-file`.
    pub fn get_demo_files(&self, branch: &str) -> Result<Vec<PathOrUrl>, PathOrUrlParseError> {
        let branch_url = format!(
            "{base}/{branch}/{demos}",
            base = DEMOS_REPOSITORY_URL_BASE,
            demos = DEMOS_REPOSITORY_DEMOS_SUBPATH
        );

        let mut files = get_files(&branch_url, ENV_KEY_DEMO_FILES)?;

        let arg_files = self.files.demo_files.clone().into_paths_or_urls()?;
        files.extend(arg_files);

        Ok(files)
    }

    /// Returns a list of stack files, consisting of entries which are either a path or URL. The list of files combines
    /// the default stack file URL constructed from [`DEMOS_REPOSITORY_URL_BASE`] and the provided branch, files provided
    /// by the ENV variable [`ENV_KEY_STACK_FILES`], and lastly, files provided by the CLI argument `--stack-file`.
    pub fn get_stack_files(&self, branch: &str) -> Result<Vec<PathOrUrl>, PathOrUrlParseError> {
        let branch_url = format!(
            "{base}/{branch}/{stacks}",
            base = DEMOS_REPOSITORY_URL_BASE,
            stacks = DEMOS_REPOSITORY_STACKS_SUBPATH
        );

        let mut files = get_files(&branch_url, ENV_KEY_STACK_FILES)?;

        let arg_files = self.files.stack_files.clone().into_paths_or_urls()?;
        files.extend(arg_files);

        Ok(files)
    }

    /// Returns a list of release files, consisting of entries which are either a path or URL. The list of files
    /// combines the default demo file URL, [`REMOTE_RELEASE_FILE`], files provided by the ENV variable
    /// [`ENV_KEY_RELEASE_FILES`], and lastly, files provided by the CLI argument `--release-file`.
    pub fn get_release_files(&self) -> Result<Vec<PathOrUrl>, PathOrUrlParseError> {
        let mut files = get_files(REMOTE_RELEASE_FILE, ENV_KEY_RELEASE_FILES)?;

        let arg_files = self.files.release_files.clone().into_paths_or_urls()?;
        files.extend(arg_files);

        Ok(files)
    }

    /// Adds the default (or custom) Helm repository URLs. Internally this calls the Helm SDK written in Go through the
    /// `go-helm-wrapper`.
    pub fn add_helm_repos(&self) -> Result<(), helm::Error> {
        tracing::info!("Add Helm repos");

        // Stable repository
        helm::add_repo(HELM_REPO_NAME_STABLE, &self.repos.helm_repo_stable)?;

        // Test repository
        helm::add_repo(HELM_REPO_NAME_TEST, &self.repos.helm_repo_test)?;

        // Dev repository
        helm::add_repo(HELM_REPO_NAME_DEV, &self.repos.helm_repo_dev)?;

        Ok(())
    }

    pub fn cache_settings(&self) -> Result<Settings, CacheSettingsError> {
        if self.no_cache {
            tracing::debug!("Cache disabled");
            Ok(Settings::disabled())
        } else {
            let project_dir = ProjectDirs::from(
                USER_DIR_QUALIFIER,
                USER_DIR_ORGANIZATION_NAME,
                USER_DIR_APPLICATION_NAME,
            )
            .ok_or(CacheSettingsError::UserDir)?;

            let cache_dir = project_dir.cache_dir();
            tracing::debug!(
                cache_dir = %cache_dir.to_string_lossy(),
                "Setting cache directory"
            );
            Ok(Settings::disk(cache_dir))
        }
    }

    #[instrument(skip_all)]
    pub async fn run(&self) -> Result<String, Error> {
        // FIXME (Techassi): There might be a better way to handle this with
        // the match later in this function.

        // Add Helm repos only when required
        match &self.subcommand {
            Commands::Completions(_) => (),
            Commands::Cache(_) => (),
            _ => self.add_helm_repos().context(HelmSnafu)?,
        }

        let cache = self
            .cache_settings()
            .unwrap()
            .try_into_cache()
            .await
            .unwrap();

        // TODO (Techassi): Do we still want to auto purge when running cache commands?
        cache.auto_purge().await.unwrap();

        match &self.subcommand {
            Commands::Operator(args) => args.run(self).await.context(OperatorSnafu),
            Commands::Release(args) => args.run(self, cache).await.context(ReleaseSnafu),
            Commands::Stack(args) => args.run(self, cache).await.context(StackSnafu),
            Commands::Stacklet(args) => args.run(self).await.context(StackletSnafu),
            Commands::Demo(args) => args.run(self, cache).await.context(DemoSnafu),
            Commands::Completions(args) => args.run().context(CompletionsSnafu),
            Commands::Cache(args) => args.run(self, cache).await.context(CacheSnafu),
            Commands::ExperimentalDebug(args) => args.run(self).await.context(DebugSnafu),
        }
    }

    // Output utility functions
    pub fn result(&self) -> Output<ResultContext> {
        Output::new(ResultContext::default(), true).expect("Failed to create output renderer")
    }

    pub fn error(&self) -> Output<ErrorContext> {
        Output::new(ErrorContext::default(), true).expect("Failed to create output renderer")
    }

    pub fn chart_type(&self) -> ChartSourceTypeArg {
        self.repos.chart_source.clone()
    }
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Interact with single operator instead of the full platform
    #[command(alias("op"))]
    Operator(operator::OperatorArgs),

    /// Interact with all operators of the platform which are released together
    #[command(alias("re"))]
    Release(release::ReleaseArgs),

    /// Interact with stacks, which are ready-to-use product combinations
    #[command(alias("st"))]
    Stack(stack::StackArgs),

    /// Interact with deployed stacklets, which are bundles of resources and
    /// containers required to run the product.
    #[command(aliases(["stl", "sl"]))]
    #[command(
        long_about = "Interact with deployed stacklets, which are bundles of resources and containers
required to run the product.

Each stacklet consists of init containers, app containers, sidecar containers
and additional Kubernetes resources like StatefulSets, ConfigMaps, Services and
CRDs."
    )]
    Stacklet(stacklet::StackletArgs),

    /// Interact with demos, which are end-to-end usage demonstrations of the Stackable data platform
    Demo(demo::DemoArgs),

    /// Generate shell completions for this tool
    #[command(alias("comp"))]
    Completions(completions::CompletionsArgs),

    /// Interact with locally cached files
    Cache(cache::CacheArgs),

    /// EXPERIMENTAL: Launch a debug container for a Pod
    #[command(long_about = "EXPERIMENTAL: Launch a debug container for a Pod.

This container will have access to the same data volumes as the primary container.")]
    ExperimentalDebug(debug::DebugArgs),
}

#[derive(Clone, Debug, Default, ValueEnum)]
pub enum OutputType {
    /// Print output formatted as plain text
    Plain,

    /// Print output formatted as a table
    #[default]
    Table,

    /// Print output formatted as JSON
    Json,

    /// Print output formatted as YAML
    Yaml,
}

#[derive(Debug, Snafu)]
#[snafu(module)]
pub enum CacheSettingsError {
    #[snafu(display("unable to resolve user directories"))]
    UserDir,
}

/// Returns a list of paths or urls based on the default (remote) file and
/// files provided via the env variable.
fn get_files(default_file: &str, env_key: &str) -> Result<Vec<PathOrUrl>, PathOrUrlParseError> {
    let mut files: Vec<PathOrUrl> = vec![default_file.into_path_or_url()?];

    let env_files = match env::var(env_key) {
        Ok(env_files) => env_files.parse_paths_or_urls()?,
        Err(_) => vec![],
    };
    files.extend(env_files);

    Ok(files)
}

/// Enum used for resolving the argument for chart source type. This will be
/// mapped to ChartSourceType (see below): the reason why we don't have one
/// enum is to avoid having to add clap dependencies to stackable-cockpit
/// for the ValueEnum macro.
#[derive(Clone, Debug, Default, ValueEnum)]
pub enum ChartSourceTypeArg {
    /// OCI registry
    #[default]
    OCI,

    /// index.yaml-based repositories: resolution (dev, test, stable) is based on the version and thus will be operator-specific
    Repo,
}

impl From<ChartSourceTypeArg> for ChartSourceType {
    /// Resolves the enum used by clap/arg-resolution to the core type used in
    /// stackable-cockpit. For the (index.yaml-based) repo case this core type cannot be
    /// decorated with meaningful information as that would be operator-specific
    /// i.e. we cannot resolve *which* (index.yaml-based) repo to use until we have inspected
    /// the operator version. Hence just a simple mapping.
    fn from(cli_enum: ChartSourceTypeArg) -> Self {
        match cli_enum {
            ChartSourceTypeArg::OCI => ChartSourceType::OCI,
            ChartSourceTypeArg::Repo => ChartSourceType::Repo,
        }
    }
}
