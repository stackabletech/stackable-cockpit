use std::process::Command;

use crate::{
    cluster::ClusterError,
    constants::{DEFAULT_LOCAL_CLUSTER_NAME, DEFAULT_STACKABLE_NAMESPACE},
    utils::path::binaries_present,
};

pub struct MinikubeCluster {
    namespace: String,
    node_count: usize,
    name: String,
}

impl MinikubeCluster {
    /// Create a new kind cluster. This will NOT yet create the cluster on the system, but instead will return a data
    /// structure representing the cluster. To actually create the cluster, the `create` method must be called.
    pub fn new(node_count: usize, name: Option<String>, namespace: Option<String>) -> Self {
        Self {
            namespace: namespace.unwrap_or(DEFAULT_STACKABLE_NAMESPACE.into()),
            name: name.unwrap_or(DEFAULT_LOCAL_CLUSTER_NAME.into()),
            node_count,
        }
    }

    /// Create a new local cluster by calling the minikube binary
    pub fn create(&self) -> Result<(), ClusterError> {
        // Check if required binaries are present
        if !binaries_present(["docker", "minikube"]) {
            return Err(ClusterError::MissingDeps);
        }

        // Create local cluster via minikube
        let minikube_cmd = Command::new("minikube")
            .arg("start")
            .args(["--nodes", self.node_count.to_string().as_str()])
            .args(["--namespace", self.namespace.as_str()])
            .args(["-p", self.name.as_str()])
            .status();

        if let Err(err) = minikube_cmd {
            return Err(ClusterError::Cmd(err.to_string()));
        }

        Ok(())
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
}
