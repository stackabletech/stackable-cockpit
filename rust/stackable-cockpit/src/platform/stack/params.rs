use stackable_operator::kvp::Labels;

#[derive(Debug)]
pub struct StackInstallParameters {
    pub demo_name: Option<String>,
    pub stack_name: String,

    pub operator_namespace: String,
    pub product_namespace: String,

    pub parameters: Vec<String>,
    pub skip_release: bool,
    pub labels: Labels,
    pub use_registry: bool,
}
