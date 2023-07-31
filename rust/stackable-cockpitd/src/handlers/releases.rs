use axum::{extract::Path, routing::get, Json, Router};
use stackable_cockpit::platform::release::ReleaseSpec;

/// Creates the release sub-router.
pub fn router() -> Router {
    Router::new()
        .route("/", get(get_releases))
        .route("/:name", get(get_release))
}

/// Retrieves all releases.
#[utoipa::path(get, path = "/releases", responses(
    (status = 200, description = "Retrieving a list of releases succeeded", body = [ReleaseSpec]),
    (status = 404, description = "Retrieving a list of releases failed")
))]
pub async fn get_releases() -> Json<Vec<ReleaseSpec>> {
    todo!()
}

/// Retrieves one release identified by `name`.
#[utoipa::path(get, path = "/releases/{name}")]
pub async fn get_release(Path(_name): Path<String>) -> Json<ReleaseSpec> {
    todo!()
}
