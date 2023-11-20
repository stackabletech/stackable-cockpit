use axum::{extract::Path, routing::get, Json, Router};
use stackable_cockpit::platform::demo::DemoSpec;

/// Creates the demo sub-router.
pub fn router() -> Router {
    Router::new()
        .route("/", get(get_demos))
        .route("/:name", get(get_demo))
}

/// Retrieves all demos.
#[utoipa::path(get, path = "/demos", responses(
    (status = 200, description = "Retrieving a list of demos succeeded", body = [DemoSpecV2]),
    (status = 404, description = "Retrieving a list of demos failed")
))]
pub async fn get_demos() -> Json<Vec<DemoSpec>> {
    todo!()
}

/// Retrieves one demo identified by `name`.
#[utoipa::path(get, path = "/demos/{name}", responses(
    (status = 200, description = "Retrieving the demo with 'name' succeeded", body = DemoSpecV2),
    (status = 404, description = "Retrieving the demo with 'name' failed")
))]
pub async fn get_demo(Path(_name): Path<String>) -> Json<DemoSpec> {
    todo!()
}
