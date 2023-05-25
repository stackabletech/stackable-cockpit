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
        handlers::get_demos,
        handlers::get_demo,
        handlers::get_releases,
        handlers::get_release,
        handlers::get_stacklets
    ),
    components(schemas(DemoSpecV2, ManifestSpec, Parameter, ReleaseSpec, handlers::Stacklet, handlers::ObjectMeta))
)]
pub struct ApiDoc {}
