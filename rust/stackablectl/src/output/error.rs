use stackable_cockpit::constants::{DEFAULT_OPERATOR_NAMESPACE, DEFAULT_PRODUCT_NAMESPACE};

use crate::output::ContextExt;

#[derive(Debug, Default)]
pub struct ErrorContext {
    pub post_hints: Vec<String>,
    pub pre_hints: Vec<String>,

    pub error_report: String,
}

impl ContextExt for ErrorContext {
    fn into_context(self) -> tera::Context {
        let mut ctx = tera::Context::new();

        ctx.insert("default_operator_namespace", DEFAULT_OPERATOR_NAMESPACE);
        ctx.insert("default_product_namespace", DEFAULT_PRODUCT_NAMESPACE);

        ctx.insert("post_hints", &self.post_hints);
        ctx.insert("pre_hints", &self.pre_hints);

        ctx.insert("error_report", &self.error_report);

        ctx
    }
}
