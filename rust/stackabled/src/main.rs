use std::net::SocketAddr;

use axum::{
    response::Redirect,
    routing::{get, post},
    Router, Server,
};
use clap::Parser;
use futures::FutureExt;
use snafu::{ResultExt, Whatever};
use stackabled::{
    api_doc, handlers,
    middleware::{self, authentication::Authenticator},
};
use utoipa_swagger_ui::SwaggerUi;

use crate::cli::Cli;

mod cli;

#[tokio::main]
#[snafu::report]
async fn main() -> Result<(), Whatever> {
    let cli = Cli::parse();

    let authn =
        Authenticator::load_htpasswd(&cli.htpasswd).whatever_context("failed to load htpasswd")?;

    // Run the server
    let api = Router::new()
        .route("/ping", get(handlers::root::ping))
        .nest("/demos", handlers::demos::router())
        .nest("/stacks", handlers::stacks::router())
        .nest("/releases", handlers::releases::router())
        .nest("/stacklets", handlers::stacklets::router())
        .route("/login", post(middleware::authentication::log_in))
        .layer(authn.clone().layer());

    let router = Router::new()
        .nest("/api/", api)
        .nest("/ui/", handlers::ui::router())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api_doc::openapi()))
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

    Ok(())
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
