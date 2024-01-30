pub mod check;
pub mod k8s;
pub mod params;
pub mod path;
pub mod string;
pub mod templating;
pub mod url;

/// Returns the name of the operator used in the Helm repository.
pub fn operator_chart_name(name: &str) -> String {
    format!("{}-operator", name)
}
