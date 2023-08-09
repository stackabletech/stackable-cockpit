use clap::{CommandFactory, Parser};
use clap_complete::{generate, Shell};
use clap_mangen::Man;
use snafu::{ensure, ResultExt, Snafu};
use stackable_cockpitd::api_doc::openapi;
use stackablectl::cli::Cli;

use std::{
    fs,
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

const USAGE_STRING: &str = "Command line tool to interact with a Stackable Data Platform\n\nUsage: stackablectl [OPTIONS] <COMMAND>\n";

#[derive(clap::Parser)]
#[allow(clippy::enum_variant_names)]
enum XtaskCommand {
    GenMan,
    GenComp,
    GenOpenapi,
    GenCtlReadme,
}

#[derive(Debug, Snafu)]
enum TaskError {
    #[snafu(display("io error"))]
    Io { source: std::io::Error },

    #[snafu(display("error serializing openapi"))]
    SerializeOpenApi { source: serde_json::Error },

    #[snafu(display("error running importing openapi schema importer"))]
    ImportOpenapiSchemaRun { source: std::io::Error },

    #[snafu(display("openapi schema importer failed with error code {error_code:?}"))]
    ImportOpenapiSchema { error_code: Option<i32> },

    #[snafu(display("error writing openapi schema into importer"))]
    WriteOpenapiSchema { source: std::io::Error },
}

#[snafu::report]
fn main() -> Result<(), TaskError> {
    match XtaskCommand::parse() {
        XtaskCommand::GenMan => {
            let cmd = Cli::command();

            fs::create_dir_all("extra/man").context(IoSnafu)?;
            let mut f = fs::File::create("extra/man/stackablectl.1").context(IoSnafu)?;

            let man = Man::new(cmd);
            man.render(&mut f).context(IoSnafu)?
        }
        XtaskCommand::GenComp => {
            let mut cmd = Cli::command();
            let name = cmd.get_name().to_string();

            fs::create_dir_all("extra/completions").context(IoSnafu)?;

            // Bash completions
            let mut f = fs::File::create("extra/completions/stackablectl.bash").context(IoSnafu)?;
            generate(Shell::Bash, &mut cmd, name.clone(), &mut f);

            // Fish completions
            let mut f = fs::File::create("extra/completions/stackablectl.fish").context(IoSnafu)?;
            generate(Shell::Fish, &mut cmd, name.clone(), &mut f);

            // ZSH completions
            let mut f = fs::File::create("extra/completions/_stackablectl").context(IoSnafu)?;
            generate(Shell::Zsh, &mut cmd, name, &mut f);
        }
        XtaskCommand::GenOpenapi => {
            let openapi_json = openapi().to_json().context(SerializeOpenApiSnafu)?;
            let mut codegen = Command::new("yarn")
                .args(["--cwd", "web", "run", "openapi-codegen"])
                .stdin(Stdio::piped())
                .spawn()
                .context(ImportOpenapiSchemaRunSnafu)?;
            codegen
                .stdin
                .take()
                .expect("child stdin must be available")
                .write_all(openapi_json.as_bytes())
                .context(WriteOpenapiSchemaSnafu)?;
            let status = codegen.wait().context(ImportOpenapiSchemaRunSnafu)?;
            ensure!(
                status.success(),
                ImportOpenapiSchemaSnafu {
                    error_code: status.code()
                }
            );
        }
        XtaskCommand::GenCtlReadme => {
            let mut cmd = Cli::command();
            let usage_text = cmd.render_long_help().to_string();
            let usage_text: Vec<_> = usage_text.lines().map(|l| l.trim_end()).collect();
            let usage_text = usage_text.join("\n");

            let readme_path = Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .join("stackablectl/README.md");

            let mut readme = fs::read_to_string(&readme_path).context(IoSnafu)?;
            let usage_start = readme.find(USAGE_STRING).unwrap();
            let usage_end = readme[usage_start..].find("\n```").unwrap();

            readme.replace_range(usage_start..usage_start + usage_end, &usage_text);
            fs::write(readme_path, readme).context(IoSnafu)?
        }
    }

    Ok(())
}
