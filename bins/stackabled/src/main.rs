use std::net::SocketAddr;

use anyhow::Result;
use axum::{routing::get, Router, Server};
use clap::Parser;

use crate::cli::Cli;

mod cli;
mod handlers;

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Run the server
    run(&cli)
}

#[tokio::main]
async fn run(cli: &Cli) -> Result<()> {
    let router = Router::new()
        .route("/", get(handlers::get_root))
        .nest("/demos", handlers::demo_router())
        .nest("/stacks", handlers::stack_router())
        .nest("/releases", handlers::release_router());

    // Needed in next axum version
    // let listener = TcpListener::bind("127.0.0.1:8000").await?;

    Server::bind(&SocketAddr::new(cli.address, cli.port))
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
