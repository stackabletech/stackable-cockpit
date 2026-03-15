use serde_yaml::Mapping;
use stackable_operator::kvp::Labels;

use crate::platform::operator::ChartSourceType;

pub struct DemoInstallParameters {
    pub demo_name: String,

    pub operator_namespace: String,
    pub demo_namespace: String,

    pub stack_parameters: Vec<String>,
    pub parameters: Vec<String>,
    pub skip_release: bool,

    pub stack_labels: Labels,
    pub labels: Labels,
    pub chart_source: ChartSourceType,
    pub operator_values: Mapping,
}

pub struct DemoUninstallParameters {
    pub demo_name: String,

    pub operator_namespace: String,
    pub demo_namespace: String,

    pub skip_operators: bool,
    pub skip_crds: bool,
}
