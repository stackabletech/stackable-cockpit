use stackable_cockpit::constants::{DEFAULT_OPERATOR_NAMESPACE, DEFAULT_PRODUCT_NAMESPACE};

use crate::output::{ContextExt, ErrorReport, OutputKind};

#[derive(Debug, Default)]
pub struct ErrorContext {
    post_hints: Vec<String>,
    pre_hints: Vec<String>,

    error_report: String,
    no_color: bool,
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

    fn output_kind(&self) -> OutputKind {
        OutputKind::Error
    }

    fn set_no_color(&mut self, no_color: bool) {
        self.no_color = no_color
    }
}

impl ErrorContext {
    pub fn with_error_report(&mut self, error: impl ErrorReport) -> &mut Self {
        // We use expect here because we want to render the error report. If
        // this fails, there is no point in catching an error while trying to
        // render a different error.
        self.error_report = error
            .into_error_report()
            .expect("failed to render error report");
        self
    }

    pub fn with_post_hint(&mut self, post_hint: impl Into<String>) -> &mut Self {
        self.post_hints.push(post_hint.into());
        self
    }
}
