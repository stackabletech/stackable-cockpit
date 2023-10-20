use std::{
    fmt::Write,
    ops::{Deref, DerefMut},
};

use snafu::{ResultExt, Snafu};
use spinoff::{spinners, Color, Spinner};
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
        write!(
            report,
            "{}",
            color_print::cformat!("An unrecoverable error occured: <s><r>{}</></>\n\n", self)
        )?;
        writeln!(
            report,
            "Caused by these errors (recent errors listed first):"
        )?;

        let mut error: &dyn std::error::Error = &self;
        let mut index = 1;

        while let Some(source) = error.source() {
            let source_string = source.to_string();

            let cleaned = if let Some((cleaned, _)) = source_string.split_once(':') {
                cleaned
            } else {
                &source_string
            };

            writeln!(
                report,
                "{}",
                color_print::cformat!(" {}: <r>{}</>", index, cleaned)
            )?;

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
    progress: Option<Spinner>,
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

        Ok(Self {
            progress: None,
            renderer,
            context,
        })
    }

    pub fn enable_progress(&mut self, initial_message: String) {
        self.progress
            .get_or_insert(Spinner::new(spinners::Dots, initial_message, Color::Green));
    }

    pub fn set_progress_message(&mut self, message: impl Into<String>) {
        if let Some(progress) = self.progress.as_mut() {
            progress.update_text(message.into())
        }
    }

    pub fn finish_progress(&mut self, message: impl AsRef<str>) {
        if let Some(progress) = self.progress.as_mut() {
            progress.success(message.as_ref())
        }
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
