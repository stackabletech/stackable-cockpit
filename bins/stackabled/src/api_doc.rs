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
    components(schemas(DemoSpecV2, ManifestSpec, Parameter, ReleaseSpec, handlers::stacklets::Stacklet, synthetic_types::ObjectMeta))
)]
pub struct ApiDoc {}

/// Synthetic types that are used to generate type definitions for foreign types.
mod synthetic_types {

    use utoipa::ToSchema;

    #[derive(ToSchema)]
    pub struct ObjectMeta {
        pub name: String,
        pub namespace: String,
    }
}
