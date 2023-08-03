use clap::{Args, ValueHint};

use crate::constants::{HELM_REPO_URL_DEV, HELM_REPO_URL_STABLE, HELM_REPO_URL_TEST};

#[derive(Debug, Args)]
#[command(next_help_heading = "Helm repository options")]
pub struct CommonRepoArgs {
    // Provide a custom Helm stable repository URL
    #[arg(long, value_name = "URL", value_hint = ValueHint::Url, default_value = HELM_REPO_URL_STABLE, global = true)]
    pub helm_repo_stable: String,

    /// Provide a custom Helm test repository URL
    #[arg(long, value_name = "URL", value_hint = ValueHint::Url, default_value = HELM_REPO_URL_TEST, global = true)]
    pub helm_repo_test: String,

    /// Provide a custom Helm dev repository URL
    #[arg(long, value_name = "URL", value_hint = ValueHint::Url, default_value = HELM_REPO_URL_DEV, global = true)]
    pub helm_repo_dev: String,
}
