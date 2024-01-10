#[derive(Debug)]
pub struct StackInstallParameters {
    pub demo_name: Option<String>,
    pub stack_name: String,

    pub operator_namespace: String,
    pub product_namespace: String,

    pub skip_release: bool,
}
