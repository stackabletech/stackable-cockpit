use serde_yaml::Mapping;
use stackable_operator::kvp::Labels;

use crate::platform::operator::ChartSourceType;

#[derive(Debug)]
pub struct StackInstallParameters {
    pub stack_name: String,

    pub operator_namespace: String,
    pub stack_namespace: String,

    pub parameters: Vec<String>,
    pub skip_release: bool,
    pub labels: Labels,
    pub chart_source: ChartSourceType,
    pub operator_values: Mapping,
}

pub struct StackUninstallParameters {
    pub stack_name: String,

    pub operator_namespace: String,
    pub stack_namespace: String,

    pub skip_operators: bool,
    pub skip_crds: bool
}
