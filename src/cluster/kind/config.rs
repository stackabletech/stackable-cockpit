use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KindClusterConfig {
    kind: String,
    api_version: String,
    nodes: Vec<KindClusterNodeConfig>,
}

impl Default for KindClusterConfig {
    fn default() -> Self {
        Self {
            kind: "Cluster".into(),
            api_version: "kind.x-k8s.io/v1alpha4".into(),
            nodes: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KindClusterNodeConfig {
    role: NodeRole,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum NodeRole {
    Worker,
    ControlPlane,
}

pub enum ControlPlaneStrategy {
    OnlyOne,
    Balanced,
}

impl KindClusterConfig {
    pub fn new(node_count: usize, control_plane_strategy: ControlPlaneStrategy) -> Self {
        let control_plane_node_count = match control_plane_strategy {
            ControlPlaneStrategy::OnlyOne => 1,
            ControlPlaneStrategy::Balanced => node_count / 2,
        };

        // Create control plane nodes
        let mut control_plane_nodes = Vec::new();

        for _ in 0..control_plane_node_count {
            control_plane_nodes.push(KindClusterNodeConfig {
                role: NodeRole::ControlPlane,
            });
        }

        // Create worker nodes
        let mut worker_nodes = Vec::new();

        for _ in 0..node_count - control_plane_node_count {
            worker_nodes.push(KindClusterNodeConfig {
                role: NodeRole::Worker,
            })
        }

        Self {
            nodes: [control_plane_nodes, worker_nodes].concat(),
            ..Default::default()
        }
    }
}
