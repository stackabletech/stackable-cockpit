use std::{
    io::Write,
    process::{Command, Stdio},
};

use snafu::{ensure, ResultExt, Snafu};
use tracing::{debug, info, instrument};

use crate::{
    cluster::{check_if_docker_is_running, kind::config::KindClusterConfig, DockerError},
    constants::{DEFAULT_LOCAL_CLUSTER_NAME, DEFAULT_STACKABLE_NAMESPACE},
    utils::check::binaries_present,
};

mod config;

#[derive(Debug, Snafu)]
pub enum ClusterError {
    #[snafu(display("io error: {source}"))]
    IoError { source: std::io::Error },

    #[snafu(display("stdin error"))]
    StdinError,

    #[snafu(display("command error: {error}"))]
    CmdError { error: String },

    #[snafu(display("missing dependencies"))]
    MissingDepsError,

    #[snafu(display("Docker error: {source}"))]
    DockerError { source: DockerError },

    #[snafu(display("yaml error: {source}"))]
    YamlError { source: serde_yaml::Error },
}

#[derive(Debug)]
pub struct KindCluster {
    cp_node_count: usize,
    namespace: String,
    node_count: usize,
    name: String,
}

impl KindCluster {
    /// Create a new kind cluster. This will NOT yet create the cluster on the system, but instead will return a data
    /// structure representing the cluster. To actually create the cluster, the `create` method must be called.
    pub fn new(
        node_count: usize,
        cp_node_count: usize,
        name: Option<String>,
        namespace: Option<String>,
    ) -> Self {
        Self {
            namespace: namespace.unwrap_or(DEFAULT_STACKABLE_NAMESPACE.into()),
            name: name.unwrap_or(DEFAULT_LOCAL_CLUSTER_NAME.into()),
            cp_node_count,
            node_count,
        }
    }

    /// Create a new local cluster by calling the kind binary.
    #[instrument]
    pub fn create(&self) -> Result<(), ClusterError> {
        info!("Creating local cluster using kind");

        // Check if required binaries are present
        if !binaries_present(["docker", "kind"]) {
            return Err(ClusterError::MissingDepsError);
        }

        // Check if Docker is running
        check_if_docker_is_running().context(DockerSnafu {})?;

        debug!("Creating kind cluster config");
        let config = KindClusterConfig::new(self.node_count, self.cp_node_count);
        let config_string = serde_yaml::to_string(&config).context(YamlSnafu {})?;

        // println!("{config_string}");

        debug!("Creating kind cluster");
        let mut kind_cmd = Command::new("kind")
            .args(["create", "cluster"])
            .args(["--name", self.name.as_str()])
            .args(["--config", "-"])
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .context(IoSnafu {})?;

        kind_cmd
            .stdin
            .as_ref()
            .ok_or(ClusterError::StdinError)?
            .write_all(config_string.as_bytes())
            .context(IoSnafu {})?;

        if let Err(err) = kind_cmd.wait() {
            return Err(ClusterError::CmdError {
                error: err.to_string(),
            });
        }

        Ok(())
    }

    /// Creates a kind cluster if it doesn't exist already.
    #[instrument]
    pub fn create_if_not_exists(&self) -> Result<(), ClusterError> {
        info!("Creating cluster if it doesn't exist using kind");

        if Self::check_if_cluster_exists(&self.name)? {
            return Ok(());
        }

        self.create()
    }

    /// Retrieve the cluster namespace
    pub fn get_namespace(&self) -> &String {
        &self.namespace
    }

    /// Retrieve the cluster node count
    pub fn get_node_count(&self) -> usize {
        self.node_count
    }

    /// Retrieve the cluster name
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// Cheack if a kind cluster with the provided name already exists.
    #[instrument]
    fn check_if_cluster_exists<T>(cluster_name: T) -> Result<bool, ClusterError>
    where
        T: AsRef<str> + std::fmt::Debug,
    {
        debug!("Cheacking if kind cluster exists");

        let output = Command::new("kind")
            .args(["get", "clusters"])
            .output()
            .context(IoSnafu {})?;

        ensure!(
            output.status.success(),
            CmdSnafu {
                error: String::from_utf8_lossy(&output.stderr)
            }
        );

        let output = String::from_utf8_lossy(&output.stdout);
        Ok(output.lines().any(|name| name == cluster_name.as_ref()))
    }
}
