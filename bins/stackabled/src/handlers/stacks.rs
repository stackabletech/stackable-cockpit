use axum::{extract::Path, routing::get, Json, Router};
use stackable::platform::stack::StackSpecV2;

/// Creates the stack sub-router.
pub fn stack_router() -> Router {
    Router::new()
        .route("/", get(get_stacks))
        .route("/:stack_name", get(get_stack))
}

/// Retrieves all stacks.
pub async fn get_stacks() -> Json<Vec<StackSpecV2>> {
    todo!()
}

/// Retrieves one stack identified by `stack_name`.
pub async fn get_stack(Path(_stack_name): Path<String>) -> Json<StackSpecV2> {
    todo!()
}
