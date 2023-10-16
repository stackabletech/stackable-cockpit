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
pub enum OutputKind<C> {
    Result(C),
    Error(C),
}

pub trait ContextExt {
    fn into_context(self) -> tera::Context;
}

#[derive(Debug)]
pub struct Output<C>
where
    C: ContextExt,
{
    kind: OutputKind<C>,
    no_color: bool,
    renderer: Tera,
}

impl<C> Output<C>
where
    C: ContextExt,
{
    pub fn new(kind: OutputKind<C>, no_color: bool) -> Result<Self> {
        let renderer = Self::create_renderer()?;
        let no_color = use_colored_output(!no_color);

        Ok(Self {
            no_color,
            renderer,
            kind,
        })
    }

    pub fn result(context: C, no_color: bool) -> Result<Self> {
        Self::new(OutputKind::Result(context), no_color)
    }

    pub fn error(context: C, no_color: bool) -> Result<Self> {
        Self::new(OutputKind::Error(context), no_color)
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
        match self.kind {
            OutputKind::Result(ctx) => self
                .renderer
                .render("result", &ctx.into_context())
                .context(RenderSnafu),
            OutputKind::Error(ctx) => self
                .renderer
                .render("error", &ctx.into_context())
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
        match &self.kind {
            OutputKind::Result(c) => c,
            OutputKind::Error(c) => c,
        }
    }
}

impl<C> DerefMut for Output<C>
where
    C: ContextExt,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        match &mut self.kind {
            OutputKind::Result(c) => c,
            OutputKind::Error(c) => c,
        }
    }
}
