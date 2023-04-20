use std::{
    io::Write,
    process::{Command, Stdio},
};

use crate::{
    cluster::ClusterError,
    constants::{DEFAULT_LOCAL_CLUSTER_NAME, DEFAULT_STACKABLE_NAMESPACE},
    utils::path::binaries_present,
};

const KIND_CLUSTER_DEF_HEADER: &str = r#"
kind: Cluster
apiVersion: kind.x-k8s.io/v1alpha4
nodes:
"#;

const KIND_CLUSTER_DEF_CP: &str = "- role: control-plane";

const KIND_CLUSTER_DEF_WORKER: &str = r#"
- role: worker
  kubeadmConfigPatches:
    - |
        kind: JoinConfiguration
        nodeRegistration:
        kubeletExtraArgs:
"#;

pub struct KindCluster {
    namespace: String,
    node_count: usize,
    name: String,
}

pub struct KindClusterConfig {
    node_count: usize,
}

impl KindCluster {
    /// Create a new kind cluster. This will NOT yet create the cluster on the system, but instead will return a data
    /// structure representing the cluster. To actually create the cluster, the `create` method must be called.
    pub fn new(node_count: usize, name: Option<String>, namespace: Option<String>) -> Self {
        Self {
            namespace: namespace.unwrap_or(DEFAULT_STACKABLE_NAMESPACE.into()),
            name: name.unwrap_or(DEFAULT_LOCAL_CLUSTER_NAME.into()),
            node_count,
        }
    }

    /// Create a new local cluster by calling the kind binary
    pub fn create(&self) -> Result<(), ClusterError> {
        // Check if required binaries are present
        if !binaries_present(["docker", "kind"]) {
            return Err(ClusterError::MissingDeps);
        }

        let config = KindClusterConfig::new(self.node_count);

        let kind_cmd = Command::new("kind")
            .args(["create", "cluster"])
            .args(["--name", self.name.as_str()])
            .args(["--config", "-"])
            .stdin(Stdio::piped())
            .spawn()?;

        kind_cmd
            .stdin
            .as_ref()
            .ok_or(ClusterError::Stdin)?
            .write_all(config.to_string().as_bytes())?;

        if let Err(err) = kind_cmd.wait_with_output() {
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

impl KindClusterConfig {
    pub fn new(node_count: usize) -> Self {
        Self { node_count }
    }

    pub fn to_string(&self) -> String {
        let config = format!("{}{}", KIND_CLUSTER_DEF_HEADER, KIND_CLUSTER_DEF_CP);
        let mut workers = String::new();

        for i in 0..self.node_count - 1 {
            workers.push_str(KIND_CLUSTER_DEF_WORKER);
            workers.push_str(format!("        node-labels: node={}", i + 1).as_str());
        }

        format!("{}{}", config, workers)
    }
}