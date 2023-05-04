use std::net::IpAddr;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, propagate_version = true)]
#[command(
    about = "Run the Stackable daemon which exposes the Stackable library via an HTTP
REST API and provides a web-based application."
)]
pub struct Cli {
    /// Port the daemon listens on
    #[arg(short, long, default_value_t = 8000, env = "STACKABLED_PORT")]
    pub port: u16,

    /// Address the server binds to
    #[arg(short, long, default_value = "127.0.0.1", env = "STACKABLED_ADDRESS")]
    pub address: IpAddr,
}
