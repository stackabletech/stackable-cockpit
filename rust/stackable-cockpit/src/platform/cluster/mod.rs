use k8s_openapi::api::core::v1::Node;
use kube::core::ObjectList;
use snafu::{ResultExt, Snafu};
use stackable_operator::{cpu::CpuQuantity, memory::MemoryQuantity};

mod resource_request;

pub use resource_request::*;

type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("failed to parse node cpu quantity"))]
    ParseNodeCpuQuantity {
        source: stackable_operator::error::Error,
    },

    #[snafu(display("failed to parse node memory quantity"))]
    ParseNodeMemoryQuantity {
        source: stackable_operator::error::Error,
    },
}

/// [`ClusterInfo`] contains information about the Kubernetes cluster, such as
/// the number of nodes and allocatable resources.
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
    pub fn from_nodes(nodes: ObjectList<Node>) -> Result<Self> {
        // FIXME (Techassi): Also retrieve number of control plane nodes
        let node_count = nodes.items.len();

        let untainted_nodes = nodes.into_iter().filter(|node| {
            node.spec
                .as_ref()
                .and_then(|spec| spec.taints.as_ref().map(|taints| taints.is_empty()))
                .unwrap_or(true)
        });
        let untainted_node_count = untainted_nodes.clone().count();

        let untainted_allocatable = untainted_nodes
            .into_iter()
            .filter_map(|node| node.status)
            .filter_map(|status| status.allocatable);

        let mut untainted_allocatable_memory = MemoryQuantity::from_mebi(0.0);
        let mut untainted_allocatable_cpu = CpuQuantity::from_millis(0);

        for mut node in untainted_allocatable {
            if let Some(q) = node.remove("cpu") {
                let cpu = CpuQuantity::try_from(q).context(ParseNodeCpuQuantitySnafu)?;
                untainted_allocatable_cpu += cpu;
            }

            if let Some(q) = node.remove("memory") {
                let memory = MemoryQuantity::try_from(q).context(ParseNodeMemoryQuantitySnafu)?;
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
