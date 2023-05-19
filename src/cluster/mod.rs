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
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum NodeRole {
    Worker,
    ControlPlane,
}
