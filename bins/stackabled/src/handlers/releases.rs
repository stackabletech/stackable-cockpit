use axum::{extract::Path, routing::get, Json, Router};
use stackable::platform::release::ReleaseSpec;

/// Creates the release sub-router.
pub fn release_router() -> Router {
    Router::new()
        .route("/", get(get_releases))
        .route("/:release_name", get(get_release))
}

/// Retrieves all releases.
pub async fn get_releases() -> Json<Vec<ReleaseSpec>> {
    todo!()
}

/// Retrieves one release identified by `release_name`.
pub async fn get_release(Path(release_name): Path<String>) -> Json<ReleaseSpec> {
    todo!()
}
