use std::net::SocketAddr;

use axum::{
    response::Redirect,
    routing::{get, post},
    Router, Server,
};
use clap::Parser;
use snafu::{ResultExt, Whatever};
use stackabled::middleware::{self, authentication::Authenticator};
use utoipa_swagger_ui::SwaggerUi;

use crate::cli::Cli;

mod api_doc;
mod cli;
mod handlers;

#[tokio::main]
#[snafu::report]
async fn main() -> Result<(), Whatever> {
    let cli = Cli::parse();

    let authn =
        Authenticator::load_htpasswd(&cli.htpasswd).whatever_context("failed to load htpasswd")?;

    // Run the server
    let api = Router::new()
        .route("/", get(handlers::root::get_root))
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
        .await
    {
        eprintln!("{err}")
    }

    Ok(())
}
