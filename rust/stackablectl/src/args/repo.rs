use clap::{Args, ValueHint};

use crate::{
    cli::ChartSourceTypeArg,
    constants::{HELM_REPO_URL_DEV, HELM_REPO_URL_STABLE, HELM_REPO_URL_TEST},
};

#[derive(Debug, Args)]
#[command(next_help_heading = "Helm repository options")]
pub struct CommonRepoArgs {
    /// Provide a custom Helm stable repository URL
    #[arg(
        long, value_name = "URL",
        value_hint = ValueHint::Url,
        default_value = HELM_REPO_URL_STABLE,
        global = true
    )]
    pub helm_repo_stable: String,

    /// Provide a custom Helm test repository URL
    #[arg(
        long, value_name = "URL",
        value_hint = ValueHint::Url,
        default_value = HELM_REPO_URL_TEST,
        global = true
    )]
    pub helm_repo_test: String,

    /// Provide a custom Helm dev repository URL
    #[arg(
        long,
        value_name = "URL",
        value_hint = ValueHint::Url,
        default_value = HELM_REPO_URL_DEV,
        global = true
    )]
    pub helm_repo_dev: String,

    /// Source the charts from either a OCI registry or from index.yaml-based repositories.
    #[arg(
        long,
        long_help = "Source the charts from either a OCI registry or from index.yaml-based repositories.",
        value_enum,
        default_value_t = Default::default(),
        global = true
    )]
    pub chart_source: ChartSourceTypeArg,
}
