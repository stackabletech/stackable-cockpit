use std::process::Stdio;
use tokio::process::Command;

use snafu::{ResultExt, Snafu};
use tracing::{debug, instrument};

#[derive(Debug, Snafu)]
pub enum DockerError {
    #[snafu(display("failed to start docker command"))]
    CommandFailedToStart { source: std::io::Error },

    #[snafu(display("failed to run docker command"))]
    CommandFailedToRun { source: std::io::Error },

    #[snafu(display("it seems like Docker is not running on this system"))]
    NotRunning,
}

/// Checks if Docker is running on the system
#[instrument]
pub async fn check_if_docker_is_running() -> Result<(), DockerError> {
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

    Err(DockerError::NotRunning)
}
