use std::process::Stdio;
use tokio::process::Command;

use snafu::{ResultExt, Snafu};
use tracing::{debug, instrument};

#[derive(Debug, Snafu)]
pub enum DockerError {
    #[snafu(display("failed to read stdout"))]
    StdoutRead { source: std::io::Error },

    #[snafu(display("It seems like Docker is not running on this system"))]
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
        .context(StdoutReadSnafu)?
        .wait()
        .await
        .context(StdoutReadSnafu)?
        .success()
    {
        return Ok(());
    }

    Err(DockerError::NotRunning)
}
