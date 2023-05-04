use anyhow::Result;
use axum::{routing::get, Router, Server};
use clap::Parser;

use crate::cli::Cli;

mod cli;
mod handlers;

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Run the server
    run()
}

#[tokio::main]
async fn run() -> Result<()> {
    let router = Router::new().route("/", get(handlers::get_root));

    // Needed in next axum version
    // let listener = TcpListener::bind("127.0.0.1:8000").await?;

    Server::bind(&"127.0.0.1:8000".parse()?)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
