use serde::Serialize;

pub mod docker;
pub mod kind;
pub mod minikube;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum NodeRole {
    Worker,
    ControlPlane,
}
