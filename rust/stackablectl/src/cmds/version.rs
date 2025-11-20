use std::sync::Arc;

use clap::{Args, Subcommand};
use snafu::{ResultExt, Snafu};
use stackable_cockpit::xfer;
use tracing::instrument;

use crate::{cli::Cli, release_check};

#[derive(Debug, Args)]
pub struct VersionArguments {
    #[command(subcommand)]
    subcommand: VersionCommand,
}

#[derive(Debug, Subcommand)]
pub enum VersionCommand {
    /// Check if there is a new version available.
    Check,
}

#[derive(Debug, Snafu)]
pub enum CmdError {
    #[snafu(display("failed to retrieve latest release"))]
    RetrieveLatestRelease { source: crate::release_check::Error },
}

impl VersionArguments {
    pub async fn run(&self, client: Arc<xfer::Client>) -> Result<String, CmdError> {
        match &self.subcommand {
            VersionCommand::Check => check_cmd(client).await,
        }
    }
}

#[instrument(skip_all)]
async fn check_cmd(client: Arc<xfer::Client>) -> Result<String, CmdError> {
    let output = release_check::version_notice_output(client, true, false)
        .await
        .context(RetrieveLatestReleaseSnafu)?
        .unwrap_or_default();

    let mut result = Cli::result();
    result.with_output(output);

    Ok(result.render())
}
