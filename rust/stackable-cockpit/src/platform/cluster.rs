use k8s_openapi::api::core::v1::Node;
use kube::core::ObjectList;
use stackable_operator::{cpu::CpuQuantity, memory::MemoryQuantity};

/// [`ClusterInfo`] contains information about the allocatable amount of CPUs
/// and memory. Additionally, it contains the number of worker nodes.
#[derive(Debug)]
pub struct ClusterInfo {
    pub memory: MemoryQuantity,
    pub worker_count: usize,
    pub cpus: CpuQuantity,
}

impl ClusterInfo {
    pub fn from_nodes(nodes: ObjectList<Node>) -> Self {
        // FIXME (Techassi): Also retrieve number of control plane nodes
        let worker_count = nodes.items.len();

        let allocatable = nodes
            .into_iter()
            .filter_map(|node| node.status)
            .filter_map(|status| status.allocatable);

        let cpus: CpuQuantity = allocatable
            .clone()
            .filter_map(|mut a| a.remove("cpu"))
            .map(CpuQuantity::try_from)
            .scan((), |_, x| x.ok()) // TODO (Techassi): We shouldn't skip the errors
            .sum();

        let memory: MemoryQuantity = allocatable
            .filter_map(|mut a| a.remove("memory"))
            .map(MemoryQuantity::try_from)
            .scan((), |_, x| x.ok())
            .sum();

        ClusterInfo {
            worker_count,
            memory,
            cpus,
        }
    }
}
