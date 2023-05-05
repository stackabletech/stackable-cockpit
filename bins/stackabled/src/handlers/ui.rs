use axum::{
    extract::Path,
    headers::Header,
    http::{header::CONTENT_TYPE, HeaderValue},
    response::{AppendHeaders, Html, IntoResponse},
    routing::get,
    Router,
};

const INDEX_HTML: &str = include_str!(concat!(env!("OUT_DIR"), "/web-ui/index.html"));
const ASSETS: phf::Map<&str, &[u8]> = include!(concat!(env!("OUT_DIR"), "/web-ui-asset-map.rs"));

pub fn router() -> Router {
    Router::new()
        .route("/assets/:assset", get(asset))
        .route("/", get(ui))
        .route("/*path", get(ui))
}

async fn ui() -> Html<&'static str> {
    Html(INDEX_HTML)
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
        ASSETS[&name],
    )
}
