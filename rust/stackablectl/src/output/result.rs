use stackable_cockpit::constants::{DEFAULT_NAMESPACE, DEFAULT_OPERATOR_NAMESPACE};

use crate::output::{ContextExt, OutputKind};

#[derive(Debug, Default)]
pub struct ResultContext {
    used_operator_namespace: String,
    used_product_namespace: String,

    command_hints: Vec<String>,
    post_hints: Vec<String>,
    pre_hints: Vec<String>,

    output: String,
    no_color: bool,
}

impl ContextExt for ResultContext {
    fn into_context(self) -> tera::Context {
        let mut ctx = tera::Context::new();

        ctx.insert("default_operator_namespace", DEFAULT_OPERATOR_NAMESPACE);
        ctx.insert("default_product_namespace", DEFAULT_NAMESPACE);

        ctx.insert("used_operator_namespace", &self.used_operator_namespace);
        ctx.insert("used_product_namespace", &self.used_product_namespace);

        ctx.insert("command_hints", &self.command_hints);
        ctx.insert("post_hints", &self.post_hints);
        ctx.insert("pre_hints", &self.pre_hints);

        ctx.insert("output", &self.output);

        ctx
    }

    fn output_kind(&self) -> OutputKind {
        OutputKind::Result
    }

    fn set_no_color(&mut self, no_color: bool) {
        self.no_color = no_color
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
            "Use \"{}\" to {}.",
            command.into(),
            description.into()
        ));

        self
    }
}
