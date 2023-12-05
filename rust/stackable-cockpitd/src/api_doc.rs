use stackable_cockpit::{
    common::manifest::ManifestSpec,
    platform::{
        cluster::ResourceRequests, demo, product::ProductSpec, release, stacklet::Stacklet,
    },
    utils::{k8s::DisplayCondition, params::Parameter},
};
use utoipa::{
    openapi::security::{HttpAuthScheme, SecurityScheme},
    OpenApi,
};

use crate::{
    handlers,
    middleware::{
        self,
        authentication::{Session, SessionToken},
    },
};

#[derive(Debug, OpenApi)]
#[openapi(
    info(description = "stackable-cockpitd API specification"),
    servers((url = "/api")),
    paths(
        handlers::root::ping,
        handlers::demos::get_demos,
        handlers::demos::get_demo,
        handlers::releases::get_releases,
        handlers::releases::get_release,
        handlers::stacklets::get_stacklets,
        middleware::authentication::log_in,
    ),
    components(schemas(
        demo::DemoSpec, ManifestSpec, Parameter, release::ReleaseSpec,
        Stacklet, DisplayCondition, synthetic_types::ObjectMeta,
        Session, SessionToken, ResourceRequests, ProductSpec
    ))
)]
struct ApiDoc {}

pub fn openapi() -> utoipa::openapi::OpenApi {
    let mut docs = ApiDoc::openapi();
    docs.components
        .get_or_insert_with(Default::default)
        .add_security_schemes_from_iter([
            (
                "session_token",
                SecurityScheme::Http(utoipa::openapi::security::Http::new(HttpAuthScheme::Bearer)),
            ),
            (
                "basic",
                SecurityScheme::Http(utoipa::openapi::security::Http::new(HttpAuthScheme::Basic)),
            ),
        ]);
    docs
}

/// Synthetic types that are used to generate type definitions for foreign types.
mod synthetic_types {

    use utoipa::ToSchema;

    #[derive(ToSchema)]
    pub struct ObjectMeta {
        pub name: String,
        pub namespace: String,
    }
}
