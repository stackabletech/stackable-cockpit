use serde::Serialize;
use snafu::Snafu;

mod docker;
mod kind;
mod minikube;

pub use docker::*;
pub use kind::*;
pub use minikube::*;

#[derive(Debug, Snafu)]
pub enum ClusterError {
    #[snafu(display("kind cluster error"))]
    KindClusterError { source: KindClusterError },

    #[snafu(display("minikube cluster error"))]
    MinikubeClusterError { source: MinikubeClusterError },

    #[snafu(display(
        "invalid total node count - at least two nodes in total are needed to run a local cluster"
    ))]
    InvalidTotalNodeCountError,

    #[snafu(display(
        "invalid control-plane node count - the number of control-plane nodes needs to be lower than total node count
    "))]
    InvalidControlPlaneNodeCountError,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum NodeRole {
    Worker,
    ControlPlane,
}
