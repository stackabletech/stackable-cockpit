use axum::{
    extract::Path,
    http::{header::CONTENT_TYPE, HeaderValue},
    response::{AppendHeaders, Html, IntoResponse},
    routing::get,
    Router,
};

pub fn router() -> Router {
    Router::new()
        .route("/assets/:assset", get(asset))
        .route("/", get(ui))
        .route("/*path", get(ui))
}

async fn ui() -> Html<&'static str> {
    Html(stackable_cockpit_web::INDEX_HTML)
}
async fn asset(Path(name): Path<String>) -> impl IntoResponse {
    (
        AppendHeaders([(
            CONTENT_TYPE,
            match name.split_once('.') {
                Some((_, "js")) => HeaderValue::from_static("text/javascript"),
                Some((_, "css")) => HeaderValue::from_static("text/css"),
                _ => HeaderValue::from_static("application/octet-stream"),
            },
        )]),
        stackable_cockpit_web::ASSETS[&name],
    )
}
