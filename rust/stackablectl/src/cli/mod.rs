use std::env;

use clap::{Parser, Subcommand, ValueEnum};
use directories::ProjectDirs;
use snafu::Snafu;
use tracing::{debug, instrument, Level};

use stackable_cockpit::{
    constants::{HELM_REPO_NAME_DEV, HELM_REPO_NAME_STABLE, HELM_REPO_NAME_TEST},
    helm::{self, HelmError},
    platform::demo::DemoList,
    utils::path::{
        IntoPathOrUrl, IntoPathsOrUrls, ParsePathsOrUrls, PathOrUrl, PathOrUrlParseError,
    },
    xfer::{cache::CacheSettings, FileTransferClient},
};

use crate::{
    args::{CommonFileArgs, CommonRepoArgs},
    cmds::{
        cache::CacheArgs, completions::CompletionsArgs, demo::DemoArgs, operator::OperatorArgs,
        release::ReleaseArgs, stack::StackArgs, stacklets::StackletsArgs,
    },
    constants::{
        ENV_KEY_DEMO_FILES, ENV_KEY_RELEASE_FILES, ENV_KEY_STACK_FILES, REMOTE_DEMO_FILE,
        REMOTE_RELEASE_FILE, REMOTE_STACK_FILE, USER_DIR_APPLICATION_NAME,
        USER_DIR_ORGANIZATION_NAME, USER_DIR_QUALIFIER,
    },
};

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

    /// Do not request any remote files via the network
    #[arg(long, global = true)]
    pub offline: bool,

    #[command(flatten)]
    pub files: CommonFileArgs,

    #[command(flatten)]
    pub repos: CommonRepoArgs,

    #[command(subcommand)]
    pub subcommand: Commands,
}

impl Cli {
    /// Returns a list of demo files, consisting of entries which are either a path or URL. The list of files combines
    /// the default demo file URL, [`REMOTE_DEMO_FILE`], files provided by the ENV variable [`ENV_KEY_DEMO_FILES`], and
    /// lastly, files provided by the CLI argument `--demo-file`.
    pub fn get_demo_files(&self) -> Result<Vec<PathOrUrl>, PathOrUrlParseError> {
        let mut files = get_files(REMOTE_DEMO_FILE, ENV_KEY_DEMO_FILES)?;

        let arg_files = self.files.demo_files.clone().into_paths_or_urls()?;
        files.extend(arg_files);

        Ok(files)
    }

    pub async fn get_demo_list(&self, transfer_client: &FileTransferClient) -> DemoList {
        let files = self.get_demo_files().unwrap();
        DemoList::build(&files, transfer_client).await.unwrap()
    }

    /// Returns a list of stack files, consisting of entries which are either a path or URL. The list of files combines
    /// the default stack file URL, [`REMOTE_STACK_FILE`], files provided by the ENV variable [`ENV_KEY_STACK_FILES`],
    /// and lastly, files provided by the CLI argument `--stack-file`.
    pub fn get_stack_files(&self) -> Result<Vec<PathOrUrl>, PathOrUrlParseError> {
        let mut files = get_files(REMOTE_STACK_FILE, ENV_KEY_STACK_FILES)?;

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
    #[instrument]
    pub fn add_helm_repos(&self) -> Result<(), HelmError> {
        debug!("Add Helm repos");

        // Stable repository
        helm::add_repo(HELM_REPO_NAME_STABLE, &self.repos.helm_repo_stable)?;

        // Test repository
        helm::add_repo(HELM_REPO_NAME_TEST, &self.repos.helm_repo_test)?;

        // Dev repository
        helm::add_repo(HELM_REPO_NAME_DEV, &self.repos.helm_repo_dev)?;

        Ok(())
    }

    #[instrument]
    pub fn cache_settings(&self) -> Result<CacheSettings, CacheSettingsError> {
        if self.no_cache {
            Ok(CacheSettings::disabled())
        } else {
            let project_dir = ProjectDirs::from(
                USER_DIR_QUALIFIER,
                USER_DIR_ORGANIZATION_NAME,
                USER_DIR_APPLICATION_NAME,
            )
            .ok_or(CacheSettingsError::UserDir)?;

            Ok(CacheSettings::disk(project_dir.cache_dir()))
        }
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

    /// Interact with deployed stacklets, which are bundles of resources and
    /// containers required to run the product.
    #[command(aliases(["stacklet", "stl", "sl"]))]
    #[command(
        long_about = "Interact with deployed stacklets, which are bundles of resources and containers
required to run the product.

Each stacklet consists of init containers, app containers, sidecar containers
and additional Kubernetes resources like StatefulSets, ConfigMaps, Services and
CRDs."
    )]
    Stacklets(StackletsArgs),

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
