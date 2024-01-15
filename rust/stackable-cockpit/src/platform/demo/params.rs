use stackable_operator::kvp::Labels;

pub struct DemoInstallParameters {
    pub operator_namespace: String,
    pub product_namespace: String,

    pub stack_parameters: Vec<String>,
    pub parameters: Vec<String>,
    pub skip_release: bool,

    pub stack_labels: Labels,
    pub labels: Labels,
}
