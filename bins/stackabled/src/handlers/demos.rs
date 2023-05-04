use axum::{extract::Path, routing::get, Json, Router};
use stackable::platform::demo::DemoSpecV2;

/// Creates the demo sub-router.
pub fn demo_router() -> Router {
    Router::new()
        .route("/", get(get_demos))
        .route("/:demo_name", get(get_demo))
}

/// Retrieves all demos.
pub async fn get_demos() -> Json<Vec<DemoSpecV2>> {
    todo!()
}

/// Retrieves one demo identified by `demo_name`.
pub async fn get_demo(Path(demo_name): Path<String>) -> Json<DemoSpecV2> {
    todo!()
}
