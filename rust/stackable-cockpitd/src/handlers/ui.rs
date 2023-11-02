use axum::{
    extract::Path,
    http::{header::CONTENT_TYPE, HeaderMap, HeaderValue},
    response::{Html, IntoResponse},
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

/// Adds (or replaces) the Content-Type header with the type of the served asset.
/// So far only javascript and css are supported, for all the other types
/// `application/octet-stream` will be used.
async fn asset(mut headers: HeaderMap, Path(name): Path<String>) -> impl IntoResponse {
    headers.insert(
        CONTENT_TYPE,
        if name.ends_with(".js") {
            HeaderValue::from_static("text/javascript")
        } else if name.ends_with(".css") {
            HeaderValue::from_static("text/css")
        } else {
            HeaderValue::from_static("application/octet-stream")
        },
    );
    (headers, stackable_cockpit_web::ASSETS[&name])
}
