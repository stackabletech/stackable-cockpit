use std::net::SocketAddr;

use axum::{routing::get, Router, Server};
use clap::Parser;
use stackable::{
    common::ManifestSpec,
    platform::{demo::DemoSpecV2, release::ReleaseSpec},
    utils::params::Parameter,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::cli::Cli;

mod cli;
mod handlers;

#[derive(Debug, OpenApi)]
#[openapi(
    info(description = "Stackabled API specification"),
    paths(
        handlers::get_demos,
        handlers::get_demo,
        handlers::get_releases,
        handlers::get_release
    ),
    components(schemas(DemoSpecV2, ManifestSpec, Parameter, ReleaseSpec))
)]
struct ApiDoc {}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Run the server
    let router = Router::new()
        .route("/", get(handlers::get_root))
        .nest("/demos", handlers::demo_router())
        .nest("/stacks", handlers::stack_router())
        .nest("/releases", handlers::release_router())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    // Needed in next axum version
    // let listener = TcpListener::bind("127.0.0.1:8000").await?;

    if let Err(err) = Server::bind(&SocketAddr::new(cli.address, cli.port))
        .serve(router.into_make_service())
        .await
    {
        eprintln!("{err}")
    }
}
