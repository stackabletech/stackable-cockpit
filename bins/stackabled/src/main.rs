use std::net::SocketAddr;

use api_doc::ApiDoc;
use axum::{response::Redirect, routing::get, Router, Server};
use clap::Parser;
use futures::FutureExt;
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
        .with_graceful_shutdown(wait_for_shutdown_signal())
        .await
    {
        eprintln!("{err}")
    }
}

async fn wait_for_shutdown_signal() {
    // Copied from kube::runtime::Controller::shutdown_on_signal
    futures::future::select(
        tokio::signal::ctrl_c().map(|_| ()).boxed(),
        #[cfg(unix)]
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .unwrap()
            .recv()
            .map(|_| ())
            .boxed(),
        // Assume that ctrl_c is enough on non-Unix platforms (such as Windows)
        #[cfg(not(unix))]
        futures::future::pending::<()>(),
    )
    .await;
}
