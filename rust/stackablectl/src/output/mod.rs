use snafu::{ResultExt, Snafu};
use stackable_cockpit::constants::{DEFAULT_OPERATOR_NAMESPACE, DEFAULT_PRODUCT_NAMESPACE};
use tera::Tera;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("failed to create output renderer"))]
    CreationError { source: tera::Error },

    #[snafu(display("failed to render console output"))]
    RenderError { source: tera::Error },
}

pub trait ContextExt {
    fn into_context(self) -> tera::Context;
}

pub struct Context {}

#[derive(Debug, Default)]
pub struct SuccessContext {
    pub used_operator_namespace: String,
    pub used_product_namespace: String,

    pub post_hints: Vec<String>,
    pub pre_hints: Vec<String>,

    pub output: String,
}

impl ContextExt for SuccessContext {
    fn into_context(self) -> tera::Context {
        let mut ctx = tera::Context::new();

        ctx.insert("default_operator_namespace", DEFAULT_OPERATOR_NAMESPACE);
        ctx.insert("default_product_namespace", DEFAULT_PRODUCT_NAMESPACE);

        ctx.insert("used_operator_namespace", &self.used_operator_namespace);
        ctx.insert("used_product_namespace", &self.used_product_namespace);

        ctx.insert("post_hints", &self.post_hints);
        ctx.insert("pre_hints", &self.pre_hints);

        ctx.insert("output", &self.output);

        ctx
    }
}

impl SuccessContext {
    pub fn new(output: String) -> Self {
        Self {
            output,
            ..Default::default()
        }
    }

    pub fn add_pre_hint(&mut self, pre_hint: String) -> &mut Self {
        self.pre_hints.push(pre_hint);
        self
    }

    pub fn add_post_hint(&mut self, post_hint: String) -> &mut Self {
        self.post_hints.push(post_hint);
        self
    }
}

pub struct OutputRenderer {
    renderer: Tera,
}

impl OutputRenderer {
    pub fn new() -> Result<Self> {
        let mut renderer = Tera::default();

        renderer
            .add_raw_templates(vec![
                (
                    "result_success",
                    include_str!("templates/result_success.tpl"),
                ),
                ("result_empty", include_str!("templates/result_empty.tpl")),
            ])
            .context(CreationSnafu)?;

        Ok(Self { renderer })
    }

    pub fn success(&self, context: SuccessContext) -> Result<String> {
        self.renderer
            .render("result_success", &context.into_context())
            .context(RenderSnafu)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple() {
        let renderer = OutputRenderer::new().unwrap();
        let mut context = SuccessContext::new("Eyyy yooo".into());

        context
            .add_pre_hint("This is pre hint number one".into())
            .add_pre_hint("Pre hint number two".into());

        let out = renderer.success(context).unwrap();
        println!("{out}");
    }
}
