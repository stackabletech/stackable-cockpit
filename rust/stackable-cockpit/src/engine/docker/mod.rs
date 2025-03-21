use std::process::Stdio;

use snafu::{ResultExt, Snafu};
use tokio::process::Command;
use tracing::{debug, instrument};

type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("failed to start Docker command"))]
    CommandFailedToStart { source: std::io::Error },

    #[snafu(display("failed to run Docker command"))]
    CommandFailedToRun { source: std::io::Error },

    #[snafu(display("it seems like Docker is not running on this system"))]
    NotRunning,
}

/// Checks if Docker is running on the system
#[instrument(skip_all)]
pub async fn check_if_docker_is_running() -> Result<()> {
    debug!("Checking if Docker is running");

    if Command::new("docker")
        .arg("info")
        .stdout(Stdio::null())
        .spawn()
        .context(CommandFailedToStartSnafu)?
        .wait()
        .await
        .context(CommandFailedToRunSnafu)?
        .success()
    {
        return Ok(());
    }

    Err(Error::NotRunning)
}
