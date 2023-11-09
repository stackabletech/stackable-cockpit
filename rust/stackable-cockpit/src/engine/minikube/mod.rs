use snafu::{ResultExt, Snafu};
use tokio::process::Command;
use tracing::{debug, info, instrument};

use crate::{
    constants::DEFAULT_LOCAL_CLUSTER_NAME,
    engine::{check_if_docker_is_running, DockerError},
    utils::check::binaries_present_with_name,
};

#[derive(Debug, Snafu)]
pub enum MinikubeClusterError {
    #[snafu(display(
        "failed to determine if a minikube cluster named {cluster_name} already exists"
    ))]
    CheckCluster {
        source: std::io::Error,
        cluster_name: String,
    },

    #[snafu(display("missing required binary: {binary}"))]
    MissingBinary { binary: String },

    #[snafu(display("failed to execute minikube command: {error}"))]
    CommandError { error: String },

    #[snafu(display("failed to determine if Docker is running"))]
    DockerError { source: DockerError },
}

#[derive(Debug)]
pub struct MinikubeCluster {
    node_count: usize,
    name: String,
}

impl MinikubeCluster {
    /// Create a new kind cluster. This will NOT yet create the cluster on the system, but instead will return a data
    /// structure representing the cluster. To actually create the cluster, the `create` method must be called.
    pub fn new(node_count: usize, name: Option<String>) -> Self {
        Self {
            name: name.unwrap_or(DEFAULT_LOCAL_CLUSTER_NAME.into()),
            node_count,
        }
    }

    /// Create a new local cluster by calling the minikube binary
    #[instrument]
    pub async fn create(&self) -> Result<(), MinikubeClusterError> {
        info!("Creating local cluster using minikube");

        // Check if required binaries are present
        if let Some(binary) = binaries_present_with_name(&["docker", "minikube"]) {
            return Err(MinikubeClusterError::MissingBinary { binary });
        }

        // Check if Docker is running
        check_if_docker_is_running().await.context(DockerSnafu)?;

        // Create local cluster via minikube
        debug!("Creating minikube cluster");
        let minikube_cmd = Command::new("minikube")
            .arg("start")
            .args(["--driver", "docker"])
            .args(["--nodes", self.node_count.to_string().as_str()])
            .args(["-p", self.name.as_str()])
            .status()
            .await;

        if let Err(err) = minikube_cmd {
            return Err(MinikubeClusterError::CommandError {
                error: err.to_string(),
            });
        }

        Ok(())
    }

    /// Creates a minikube cluster if it doesn't exist already.
    #[instrument]
    pub async fn create_if_not_exists(&self) -> Result<(), MinikubeClusterError> {
        info!("Creating cluster if it doesn't exist using minikube");

        if Self::check_if_cluster_exists(&self.name).await? {
            return Ok(());
        }

        self.create().await
    }

    /// Retrieve the cluster node count
    pub fn get_node_count(&self) -> usize {
        self.node_count
    }

    /// Retrieve the cluster name
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// Check if a kind cluster with the provided name already exists.
    #[instrument]
    async fn check_if_cluster_exists(cluster_name: &str) -> Result<bool, MinikubeClusterError> {
        debug!("Checking if minikube cluster exists");

        let output = Command::new("minikube")
            .arg("status")
            .args(["-p", cluster_name])
            .args(["-o", "json"])
            .output()
            .await
            .context(CheckClusterSnafu { cluster_name })?;

        if !output.status.success() {
            return Ok(false);
        }

        Ok(true)
    }
}
