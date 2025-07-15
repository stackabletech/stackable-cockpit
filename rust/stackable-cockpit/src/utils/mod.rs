pub mod chartsource;
pub mod check;
pub mod k8s;
pub mod params;
pub mod path;
pub mod templating;

/// Returns the name of the operator used in the Helm repository.
pub fn operator_chart_name(name: &str) -> String {
    format!("{name}-operator")
}
