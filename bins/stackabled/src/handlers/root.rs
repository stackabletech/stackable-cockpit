use axum::Json;

#[utoipa::path(get, path = "/ping", responses((status = 200, body = String)))]
pub async fn ping() -> Json<String> {
    Json("pong!".to_string())
}
