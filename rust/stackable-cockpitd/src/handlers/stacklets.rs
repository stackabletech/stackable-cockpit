use axum::{routing::get, Json, Router};
use stackable_cockpit::platform;

pub use stackable_cockpit::{kube::DisplayCondition, platform::stacklet::Stacklet};

/// Creates the stack sub-router.
pub fn router() -> Router {
    Router::new().route("/", get(get_stacklets))
}

/// Retrieves all stacklets.
#[utoipa::path(get, path = "/stacklets", responses(
    (status = 200, body = Vec<Stacklet>),
))]
pub async fn get_stacklets() -> Json<Vec<Stacklet>> {
    Json(platform::stacklet::list(None).await.unwrap())
}
