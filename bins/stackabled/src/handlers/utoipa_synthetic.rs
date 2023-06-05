//! Synthetic types that are used to generate type definitions for foreign types.

use utoipa::ToSchema;

#[derive(ToSchema)]
pub struct ObjectMeta {
    pub name: String,
    pub namespace: String,
}
