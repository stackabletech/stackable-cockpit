use axum::{Router, routing::get};

pub fn router() -> Router {
    Router::new().route("/", get(disabled_message))
}

async fn disabled_message() -> &'static str {
    "UI is disabled, rebuild with --features ui to enable"
}
