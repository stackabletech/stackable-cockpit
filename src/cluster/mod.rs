use snafu::Snafu;

mod kind;
mod minikube;

pub use kind::*;
pub use minikube::*;

#[derive(Debug, Snafu)]
pub enum ClusterError {
    #[snafu(display("io error: {source}"))]
    IoError { source: std::io::Error },

    #[snafu(display("stdin error"))]
    Stdin,

    #[snafu(display("command error: {error}"))]
    Cmd { error: String },

    #[snafu(display("missing dependencies"))]
    MissingDeps,
}
