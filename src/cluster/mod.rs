use thiserror::Error;

mod kind;
mod minikube;

pub use kind::*;
pub use minikube::*;

#[derive(Debug, Error)]
pub enum ClusterError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("stdin error")]
    Stdin,

    #[error("command error: {0}")]
    Cmd(String),

    #[error("missing dependencies")]
    MissingDeps,
}
