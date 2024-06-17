use snafu::{ResultExt, Snafu};
use tokio::process::Command;
use tracing::{debug, info, instrument};

use crate::{
    engine::docker::{self, check_if_docker_is_running},
    utils::check::binaries_present_with_name,
};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display(
        "failed to determine if a Minikube cluster named '{cluster_name}' already exists"
    ))]
    CheckCluster {
        source: std::io::Error,
        cluster_name: String,
    },

    #[snafu(display("missing required binary {binary:?}"))]
    MissingBinary { binary: String },

    #[snafu(display("failed to execute Minikube command"))]
    MinikubeCommand { source: std::io::Error },

    #[snafu(display("failed to determine if Docker is running"))]
    DockerCheckCommand { source: docker::Error },
}

#[derive(Debug)]
pub struct Cluster {
    node_count: usize,
    name: String,
}

impl Cluster {
    /// Create a new kind cluster. This will NOT yet create the cluster on the system, but instead will return a data
    /// structure representing the cluster. To actually create the cluster, the `create` method must be called.
    pub fn new(node_count: usize, name: String) -> Self {
        Self { node_count, name }
    }

    /// Create a new local cluster by calling the Minikube binary
    #[instrument]
    pub async fn create(&self) -> Result<(), Error> {
        info!("Creating local cluster using Minikube");

        // Check if required binaries are present
        if let Some(binary) = binaries_present_with_name(&["docker", "minikube"]) {
            return Err(Error::MissingBinary { binary });
        }

        // Check if Docker is running
        check_if_docker_is_running()
            .await
            .context(DockerCheckCommandSnafu)?;

        // Create local cluster via Minikube
        debug!("Creating Minikube cluster");
        Command::new("minikube")
            .arg("start")
            .args(["--driver", "docker"])
            .args(["--nodes", self.node_count.to_string().as_str()])
            .args(["-p", self.name.as_str()])
            .status()
            .await
            .context(MinikubeCommandSnafu)?;

        Ok(())
    }

    /// Creates a Minikube cluster if it doesn't exist already.
    #[instrument]
    pub async fn create_if_not_exists(&self) -> Result<(), Error> {
        info!("Creating cluster if it doesn't exist using Minikube");

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
    async fn check_if_cluster_exists(cluster_name: &str) -> Result<bool, Error> {
        debug!("Checking if Minikube cluster exists");

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
