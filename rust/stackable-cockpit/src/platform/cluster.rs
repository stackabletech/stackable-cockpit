use k8s_openapi::api::core::v1::Node;
use kube::core::ObjectList;
use snafu::{ResultExt, Snafu};
use stackable_operator::{cpu::CpuQuantity, memory::MemoryQuantity};

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

/// [`ClusterInfo`] contains information about the allocatable amount of CPUs
/// and memory. Additionally, it contains the number of worker nodes.
#[derive(Debug)]
pub struct ClusterInfo {
    /// All nodes of the cluster regardless of their type
    pub node_count: usize,
    /// Nodes that have no taints set, this e.g. excludes kind master nodes.
    /// The idea is, that our stacks/demos don't specify any tolerations, so these nodes are
    /// not available when installing a stack or demo.
    pub untainted_worker_count: usize,
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

        let untainted_workers = nodes.into_iter().filter(|node| {
            node.spec
                .as_ref()
                .and_then(|spec| spec.taints.as_ref().map(|taints| taints.is_empty()))
                .unwrap_or(true)
        });
        let untainted_worker_count = untainted_workers.clone().count();

        let untainted_allocatable = untainted_workers
            .filter_map(|node| node.status)
            .filter_map(|status| status.allocatable);

        let untainted_allocatable_cpu: CpuQuantity = untainted_allocatable
            .clone()
            .filter_map(|mut a| a.remove("cpu"))
            .map(CpuQuantity::try_from)
            .sum::<Result<CpuQuantity, _>>()
            .context(ParseNodeCpuSnafu)?;

        let untainted_allocatable_memory: MemoryQuantity = untainted_allocatable
            .filter_map(|mut a| a.remove("memory"))
            .map(MemoryQuantity::try_from)
            .sum::<Result<MemoryQuantity, _>>()
            .context(ParseNodeMemorySnafu)?;

        Ok(ClusterInfo {
            node_count,
            untainted_worker_count,
            untainted_allocatable_cpu,
            untainted_allocatable_memory,
        })
    }
}
