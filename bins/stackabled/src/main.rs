use std::net::SocketAddr;

use anyhow::Result;
use axum::{routing::get, Router, Server};
use clap::Parser;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::cli::Cli;

mod cli;
mod handlers;

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Run the server
    run(&cli)
}

#[derive(Debug, OpenApi)]
#[openapi(
    info(description = "Stackabled API specification"),
    paths(handlers::get_demos, handlers::get_demo)
)]
struct ApiDoc {}

#[tokio::main]
async fn run(cli: &Cli) -> Result<()> {
    let router = Router::new()
        .route("/", get(handlers::get_root))
        .nest("/demos", handlers::demo_router())
        .nest("/stacks", handlers::stack_router())
        .nest("/releases", handlers::release_router())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    // Needed in next axum version
    // let listener = TcpListener::bind("127.0.0.1:8000").await?;

    Server::bind(&SocketAddr::new(cli.address, cli.port))
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
