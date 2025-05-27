//! This module contains helper structs and functions to render the CLI output
//! based on templates. These templates allow dynamic composition of the output.
//! The output offers sections for pre, post and command hints, as well as
//! success and error output. The [`ErrorReport`] serves as an alternative to
//! snafu's [`Report`](snafu::Report).

use std::{
    fmt::Write,
    ops::{Deref, DerefMut},
};

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
    CreateRenderer { source: tera::Error },
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

pub trait ErrorReport
where
    Self: std::error::Error,
{
    fn into_error_report(self) -> std::result::Result<String, std::fmt::Error>;
}

impl<T> ErrorReport for T
where
    T: std::error::Error,
{
    fn into_error_report(self) -> std::result::Result<String, std::fmt::Error> {
        let mut report = String::new();

        // Print top most error
        write!(report, "An unrecoverable error occured: {}\n\n", self)?;
        writeln!(
            report,
            "Caused by these errors (recent errors listed first):"
        )?;

        let mut error: &dyn std::error::Error = &self;
        let mut index = 1;

        while let Some(source) = error.source() {
            writeln!(report, " {index}: {source}")?;
            error = source;
            index += 1;
        }

        Ok(report)
    }
}

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

    pub fn render(self) -> String {
        // We ignore the error. If we cannot render the output, there is
        // no point to explicitly handle the error.
        match self.context.output_kind() {
            OutputKind::Result => self
                .renderer
                .render("result", &self.context.into_context())
                .expect("Failed to render result"),
            OutputKind::Error => self
                .renderer
                .render("error", &self.context.into_context())
                .expect("Failed to render error"),
        }
    }

    fn create_renderer() -> Result<Tera> {
        let mut renderer = Tera::default();

        renderer
            .add_raw_templates(vec![
                ("result", include_str!("templates/result.tpl")),
                ("error", include_str!("templates/error.tpl")),
            ])
            .context(CreateRendererSnafu)?;

        Ok(renderer)
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
