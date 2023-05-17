use serde::Serialize;

mod docker;
mod kind;
mod minikube;

pub use docker::*;
pub use kind::*;
pub use minikube::*;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum NodeRole {
    Worker,
    ControlPlane,
}
