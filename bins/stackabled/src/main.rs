use std::net::SocketAddr;

use api_doc::ApiDoc;
use axum::{response::Redirect, routing::get, Router, Server};
use clap::Parser;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::cli::Cli;

mod api_doc;
mod cli;
mod handlers;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Run the server
    let api = Router::new()
        .route("/", get(handlers::root::get_root))
        .nest("/demos", handlers::demos::router())
        .nest("/stacks", handlers::stacks::router())
        .nest("/releases", handlers::releases::router())
        .nest("/stacklets", handlers::stacklets::router());

    let router = Router::new()
        .nest("/api/", api)
        .nest("/ui/", handlers::ui::router())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/", get(|| async { Redirect::permanent("/ui/") }));

    // Needed in next axum version
    // let listener = TcpListener::bind("127.0.0.1:8000").await?;

    if let Err(err) = Server::bind(&SocketAddr::new(cli.address, cli.port))
        .serve(router.into_make_service())
        .await
    {
        eprintln!("{err}")
    }
}
