use std::fmt::Display;

use k8s_openapi::{api::core::v1::Node, apimachinery::pkg::api::resource::Quantity};
use kube::core::ObjectList;
use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};
use stackable_operator::{cpu::CpuQuantity, memory::MemoryQuantity};
use tracing::warn;

use crate::utils::k8s::{KubeClient, KubeClientError};

#[derive(Debug, Snafu)]
pub enum ClusterError {
    #[snafu(display("failed to parse node cpu"))]
    ParseNodeCpu {
        source: stackable_operator::error::Error,
    },

    #[snafu(display("failed to parse node memory"))]
    ParseNodeMemory {
        source: stackable_operator::error::Error,
    },
}

/// [`ClusterInfo`] contains information about the Kubernetes cluster, such as the number of nodes and
/// allocatable resources.
#[derive(Debug)]
pub struct ClusterInfo {
    /// All nodes of the cluster regardless of their type
    pub node_count: usize,
    /// Nodes that have no taints set, this e.g. excludes kind master nodes.
    /// The idea is, that our stacks/demos don't specify any tolerations, so these nodes are
    /// not available when installing a stack or demo.
    pub untainted_node_count: usize,
    /// Sum of allocatable cpu resources on all untainted nodes. Please note that allocatable
    /// is comparable to the total capacity of the node, not the free capacity!
    pub untainted_allocatable_cpu: CpuQuantity,
    /// Sum of allocatable memory resources on all untainted nodes. Please note that allocatable
    /// is comparable to the total capacity of the node, not the free capacity!
    pub untainted_allocatable_memory: MemoryQuantity,
    // TODO (Techassi + sbernauer): Take actual usage of nodes in consideration
    // and calculate untainted_free_cpu and untainted_free_memory
}

impl ClusterInfo {
    pub fn from_nodes(nodes: ObjectList<Node>) -> Result<Self, ClusterError> {
        // FIXME (Techassi): Also retrieve number of control plane nodes
        let node_count = nodes.items.len();

        let untainted_nodes = nodes.into_iter().filter(|node| {
            node.spec
                .as_ref()
                .and_then(|spec| spec.taints.as_ref().map(|taints| taints.is_empty()))
                .unwrap_or(true)
        });
        let untainted_node_count = untainted_nodes.clone().count();

        let untainted_allocatable: Vec<_> = untainted_nodes
            .into_iter()
            .filter_map(|node| node.status)
            .filter_map(|status| status.allocatable)
            .collect();

        let mut untainted_allocatable_memory = MemoryQuantity::from_mebi(0.0);
        let mut untainted_allocatable_cpu = CpuQuantity::from_millis(0);

        for mut node in untainted_allocatable {
            if let Some(q) = node.remove("cpu") {
                let cpu = CpuQuantity::try_from(q).context(ParseNodeCpuSnafu)?;
                untainted_allocatable_cpu += cpu;
            }

            if let Some(q) = node.remove("memory") {
                let memory = MemoryQuantity::try_from(q).context(ParseNodeMemorySnafu)?;
                untainted_allocatable_memory += memory;
            }
        }

        Ok(ClusterInfo {
            node_count,
            untainted_node_count,
            untainted_allocatable_cpu,
            untainted_allocatable_memory,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResourceRequests {
    pub cpu: Quantity,
    pub memory: Quantity,
    pub pvc: Quantity,
}

impl Display for ResourceRequests {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CPU: {}, Memory: {}, PVC space: {}",
            self.cpu.0, self.memory.0, self.pvc.0
        )
    }
}

/// This error indicates that the ResourceRequirements of a stack or a demo
/// can not be parsed
#[derive(Debug, Snafu)]
pub enum ResourceRequestsError {
    #[snafu(display("kube error: {source}"), context(false))]
    KubeError { source: KubeClientError },

    #[snafu(display("failed to parse cpu resource requirements"))]
    ParseCpuResourceRequirements {
        source: stackable_operator::error::Error,
    },

    #[snafu(display("failed to parse memory resource requirements"))]
    ParseMemoryResourceRequirements {
        source: stackable_operator::error::Error,
    },
}

impl ResourceRequests {
    /// `object_name` should be `Stack` or `Demo`.
    pub async fn warn_when_cluster_too_small(
        &self,
        object_name: &str,
    ) -> Result<(), ResourceRequestsError> {
        let kube_client = KubeClient::new().await?;
        let cluster_info = kube_client.get_cluster_info().await?;

        let stack_cpu =
            CpuQuantity::try_from(&self.cpu).context(ParseCpuResourceRequirementsSnafu)?;
        let stack_memory =
            MemoryQuantity::try_from(&self.memory).context(ParseMemoryResourceRequirementsSnafu)?;

        if stack_cpu > cluster_info.untainted_allocatable_cpu {
            warn!(
                "{object_name} has resource requirements [{self}], but cluster seems to have only {} cores",
                cluster_info.untainted_allocatable_cpu.as_cpu_count()
            );
        }
        if stack_memory > cluster_info.untainted_allocatable_memory {
            warn!("{object_name} has resource requirements [{self}], but cluster seems to have only {} of memory",
            Quantity::from(cluster_info.untainted_allocatable_memory).0);
        }

        Ok(())
    }
}
