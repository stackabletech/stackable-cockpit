use std::{
    fs::{self},
    path::Path,
};

use clap::CommandFactory;
use snafu::{ResultExt, Snafu};
use stackablectl::cli::Cli;

const COMMANDS: &[&str] = &[
    "completions",
    "stacklet",
    "operator",
    "release",
    "stack",
    "cache",
    "demo",
    ".",
];

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

    for command_page_name in COMMANDS {
        let usage_text = if command_page_name == &"." {
            cli.render_long_help().to_string()
        } else {
            match cli.find_subcommand_mut(command_page_name) {
                Some(cmd) => cmd.render_long_help().to_string(),
                None => {
                    return Err(NoSuchSubcommandSnafu {
                        name: command_page_name.to_string(),
                    }
                    .build())
                }
            }
        };

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
                if command_page_name == &"." {
                    "index"
                } else {
                    command_page_name
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
