use stackable::{
    common::ManifestSpec,
    platform::{demo::DemoSpecV2, release::ReleaseSpec},
    utils::params::Parameter,
};
pub use utoipa::OpenApi;

use crate::handlers;

#[derive(Debug, OpenApi)]
#[openapi(
    info(description = "Stackabled API specification"),
    servers((url = "/api")),
    paths(
        handlers::demos::get_demos,
        handlers::demos::get_demo,
        handlers::releases::get_releases,
        handlers::releases::get_release,
        handlers::stacklets::get_stacklets
    ),
    components(schemas(DemoSpecV2, ManifestSpec, Parameter, ReleaseSpec, handlers::stacklets::Stacklet, handlers::utoipa_synthetic::ObjectMeta))
)]
pub struct ApiDoc {}
