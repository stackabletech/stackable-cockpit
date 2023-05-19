use clap::{Args, ValueEnum};

use stackable::{
    cluster::{ClusterError, KindCluster},
    constants::DEFAULT_LOCAL_CLUSTER_NAME,
};

#[derive(Debug, Args)]
pub struct CommonClusterArgs {
    /// Type of local cluster to use for testing
    #[arg(short = 'c', long = "cluster", value_name = "CLUSTER_TYPE")]
    #[arg(
        long_help = "If specified, a local Kubernetes cluster consisting of 4 nodes (1 for
control-plane and 3 workers) will be created for testing purposes. Currently
'kind' and 'minikube' are supported. Both require a working Docker
installation on the system."
    )]
    cluster_type: Option<ClusterType>,

    /// Name of the local cluster
    #[arg(long, default_value = DEFAULT_LOCAL_CLUSTER_NAME)]
    cluster_name: String,

    /// Number of total nodes in the local cluster
    #[arg(long, default_value_t = 2)]
    #[arg(long_help = "Number of total nodes in the local cluster

This number specifies the total number of nodes, which combines control plane
and worker nodes. The number of control plane nodes can be customized with the
--cluster-cp-nodes argument. The default number of control plane nodes is '1'.
So when specifying a total number of nodes of '4', there will be one control
plane node and three worker nodes. The minimum total cluster node count is '2'.
If a smaller number is supplied, stackablectl will abort cluster creation,
operator installation and displays an error message.")]
    cluster_nodes: usize,

    /// Number of control plane nodes in the local cluster
    #[arg(long, default_value_t = 1)]
    #[arg(long_help = "Number of control plane nodes in the local cluster

This number must be smaller than --cluster-nodes. If this is not the case,
stackablectl will abort cluster creation, operator installation and displays
an error message.")]
    cluster_cp_nodes: usize,
}

impl CommonClusterArgs {
    /// Installs a local cluster with `name` in `namespace` if needed. The user
    /// has the option to not install any local cluster. If the user chooses so
    /// the function skips creation of the local cluster. If a cluster needs to
    /// be created, the function first validates cluster node counts. If this
    /// validation fails, an error is returned.
    pub async fn install_if_needed(
        &self,
        name: Option<String>,
        namespace: Option<String>,
    ) -> Result<(), ClusterError> {
        match &self.cluster_type {
            Some(cluster_type) => {
                self.validate()?;

                match cluster_type {
                    ClusterType::Kind => {
                        let kind_cluster = KindCluster::new(
                            self.cluster_nodes,
                            self.cluster_cp_nodes,
                            name,
                            namespace,
                        );

                        // Seems like we cannot propagate the error directly using ?
                        match kind_cluster.create().await {
                            Ok(_) => Ok(()),
                            Err(err) => Err(ClusterError::KindClusterError { source: err }),
                        }
                    }
                    ClusterType::Minikube => todo!(),
                }
            }
            None => Ok(()),
        }
    }

    fn validate(&self) -> Result<(), ClusterError> {
        // We need at least two nodes in total (one control-plane node and one
        // worker node)
        if self.cluster_nodes < 2 {
            return Err(ClusterError::InvalidTotalNodeCountError);
        }

        // The cluster control-plane node count must be smaller than the total
        // node count
        if self.cluster_cp_nodes >= self.cluster_nodes {
            return Err(ClusterError::InvalidControlPlaneNodeCountError);
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Default, ValueEnum)]
pub enum ClusterType {
    /// Use a kind cluster, see 'https://docs.stackable.tech/home/getting_started.html#_installing_kubernetes_using_kind'
    #[default]
    Kind,

    /// Use a minikube cluster (CURRENTLY UNSUPPORTED)
    Minikube,
}
