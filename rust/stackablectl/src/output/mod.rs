use std::ops::{Deref, DerefMut};

use snafu::{ResultExt, Snafu};
use tera::Tera;

mod error;
mod result;

pub use error::ErrorContext;
pub use result::ResultContext;

use crate::utils::use_colored_output;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("failed to create output renderer"))]
    CreationError { source: tera::Error },

    #[snafu(display("failed to render console output"))]
    RenderError { source: tera::Error },
}

#[derive(Debug)]
pub enum OutputKind {
    Result,
    Error,
}

pub trait ContextExt {
    fn set_no_color(&mut self, no_color: bool);
    fn into_context(self) -> tera::Context;
    fn output_kind(&self) -> OutputKind;
}

#[derive(Debug)]
pub struct Output<C>
where
    C: ContextExt,
{
    renderer: Tera,
    context: C,
}

impl<C> Output<C>
where
    C: ContextExt,
{
    pub fn new(mut context: C, no_color: bool) -> Result<Self> {
        let renderer = Self::create_renderer()?;
        let no_color = use_colored_output(!no_color);
        context.set_no_color(no_color);

        Ok(Self { renderer, context })
    }

    fn create_renderer() -> Result<Tera> {
        let mut renderer = Tera::default();

        renderer
            .add_raw_templates(vec![
                ("result", include_str!("templates/result.tpl")),
                ("error", include_str!("templates/error.tpl")),
            ])
            .context(CreationSnafu)?;

        Ok(renderer)
    }

    pub fn render(self) -> Result<String> {
        match self.context.output_kind() {
            OutputKind::Result => self
                .renderer
                .render("result", &self.context.into_context())
                .context(RenderSnafu),
            OutputKind::Error => self
                .renderer
                .render("error", &self.context.into_context())
                .context(RenderSnafu),
        }
    }
}

impl<C> Deref for Output<C>
where
    C: ContextExt,
{
    type Target = C;

    fn deref(&self) -> &Self::Target {
        &self.context
    }
}

impl<C> DerefMut for Output<C>
where
    C: ContextExt,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.context
    }
}
