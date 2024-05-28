use axum::{routing::get, Json, Router};
use stackable_cockpit::{platform, utils::k8s::Client};

pub use stackable_cockpit::platform::stacklet::Stacklet;

/// Creates the stack sub-router.
pub fn router() -> Router {
    Router::new().route("/", get(get_stacklets))
}

/// Retrieves all stacklets.
// TODO: Add proper error handling
#[utoipa::path(get, path = "/stacklets", responses(
    (status = 200, body = Vec<Stacklet>),
))]
pub async fn get_stacklets() -> Json<Vec<Stacklet>> {
    let client = Client::new().await.expect("failed to construct k8s client");

    Json(
        platform::stacklet::list_stacklets(&client, None)
            .await
            .expect("failed to list stacklets"),
    )
}
