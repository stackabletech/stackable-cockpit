use utoipa::ToSchema;

/// Synthetic types that are used to generate type definitions for foreign types.

#[derive(ToSchema)]
pub struct ObjectMeta {
    pub name: String,
    pub namespace: String,
}
