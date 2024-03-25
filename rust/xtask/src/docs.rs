use std::{
    fs::{self},
    path::Path,
};

use clap::CommandFactory;
use snafu::{ResultExt, Snafu};
use stackablectl::cli::Cli;

const DOCS_BASE_PATH: &str = "docs/modules/stackablectl/partials/commands";

#[derive(Debug, Snafu)]
pub enum GenDocsError {
    #[snafu(display("No such subcommand: {name}"))]
    NoSuchSubcommand { name: String },

    #[snafu(display("io error"))]
    Io { source: std::io::Error },

    #[snafu(display("templating error"))]
    TemplateError { source: tera::Error },
}

pub fn generate() -> Result<(), GenDocsError> {
    let mut cli = Cli::command();
    cli.build();

    let mut renderer = tera::Tera::default();
    renderer
        .add_raw_template(
            "command_partial",
            include_str!("templates/command.adoc.tpl"),
        )
        .context(TemplateSnafu)?;

    for cmd in cli.get_subcommands().chain([&cli]) {
        let usage_text = cmd.clone().render_long_help().to_string();

        // Needed to remove trailing whitespaces in empty lines
        let usage_text: Vec<_> = usage_text.lines().map(|l| l.trim_end()).collect();
        let usage_text = usage_text.join("\n");

        let page_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .unwrap()
            .join(DOCS_BASE_PATH)
            .join(format!(
                "{}.adoc",
                if cmd.get_name() == cli.get_name() {
                    "index"
                } else {
                    cmd.get_name()
                }
            ));

        let mut context = tera::Context::new();
        context.insert("output", &usage_text);

        fs::write(
            page_path,
            renderer
                .render("command_partial", &context)
                .context(TemplateSnafu)?,
        )
        .context(IoSnafu)?
    }

    Ok(())
}
