pub mod check;
pub mod params;
pub mod path;
pub mod read;

/// Returns the name of the operator used in the Helm repository.
pub fn operator_chart_name(name: &str) -> String {
    format!("{}-operator", name)
}
