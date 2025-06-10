use stackable_operator::kvp::Labels;

use crate::platform::operator::ChartSourceType;

pub struct DemoInstallParameters {
    pub operator_namespace: String,
    pub demo_namespace: String,

    pub stack_parameters: Vec<String>,
    pub parameters: Vec<String>,
    pub skip_release: bool,

    pub stack_labels: Labels,
    pub labels: Labels,
    pub chart_source: ChartSourceType,
}
