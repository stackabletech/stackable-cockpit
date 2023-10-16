use nu_ansi_term::Color::Green;
use stackable_cockpit::constants::{DEFAULT_OPERATOR_NAMESPACE, DEFAULT_PRODUCT_NAMESPACE};

use crate::output::ContextExt;

#[derive(Debug, Default)]
pub struct ResultContext {
    pub used_operator_namespace: String,
    pub used_product_namespace: String,

    pub command_hints: Vec<String>,
    pub post_hints: Vec<String>,
    pub pre_hints: Vec<String>,

    pub output: String,
}

impl ContextExt for ResultContext {
    fn into_context(self) -> tera::Context {
        let mut ctx = tera::Context::new();

        ctx.insert("default_operator_namespace", DEFAULT_OPERATOR_NAMESPACE);
        ctx.insert("default_product_namespace", DEFAULT_PRODUCT_NAMESPACE);

        ctx.insert("used_operator_namespace", &self.used_operator_namespace);
        ctx.insert("used_product_namespace", &self.used_product_namespace);

        ctx.insert("command_hints", &self.command_hints);
        ctx.insert("post_hints", &self.post_hints);
        ctx.insert("pre_hints", &self.pre_hints);

        ctx.insert("output", &self.output);

        ctx
    }
}

impl ResultContext {
    pub fn with_output(&mut self, output: impl Into<String>) -> &mut Self {
        self.output = output.into();
        self
    }

    pub fn with_command_hint(
        &mut self,
        command: impl Into<String>,
        description: impl Into<String>,
    ) -> &mut Self {
        self.command_hints.push(format!(
            "Use {} to {}.",
            Green.bold().paint(format!("\"{}\"", command.into())),
            description.into()
        ));
        self
    }
}
