use axum::{extract::Path, routing::get, Json, Router};
use serde::Serialize;
use stackable::kube::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use utoipa::ToSchema;

#[derive(ToSchema, Serialize)]
pub struct Stacklet {
    metadata: ObjectMeta,
    product: String,
}

/// Creates the stack sub-router.
pub fn router() -> Router {
    Router::new().route("/", get(get_stacklets))
}

/// Retrieves all stacklets.
#[utoipa::path(get, path = "/stacklets", responses(
    (status = 200, body = Vec<Stacklet>),
))]
pub async fn get_stacklets() -> Json<Vec<Stacklet>> {
    Json(vec![
        Stacklet {
            metadata: ObjectMeta {
                name: Some("simple-nifi".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            },
            product: "nifi".to_string(),
        },
        Stacklet {
            metadata: ObjectMeta {
                name: Some("simple-hdfs".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            },
            product: "hdfs".to_string(),
        },
    ])
}
