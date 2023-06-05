use clap::{CommandFactory, Parser};
use clap_complete::{generate, Shell};
use clap_mangen::Man;
use snafu::{ResultExt, Snafu};
use stackablectl::cli::Cli;
use stackabled::api_doc::{ApiDoc, OpenApi};

use std::{
    fs,
    io::Write,
    process::{Command, Stdio},
};

#[derive(clap::Parser)]
#[allow(clippy::enum_variant_names)]
enum XtaskCli {
    GenMan,
    GenComp,
    GenOpenapi,
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
    match XtaskCli::parse() {
        XtaskCli::GenMan => {
            let cmd = Cli::command();

            fs::create_dir_all("extra/man").context(IoSnafu {})?;
            let mut f = fs::File::create("extra/man/stackablectl.1").context(IoSnafu {})?;

            let man = Man::new(cmd);
            man.render(&mut f).context(IoSnafu {})?
        }
        XtaskCli::GenComp => {
            let mut cmd = Cli::command();
            let name = cmd.get_name().to_string();

            fs::create_dir_all("extra/completions").context(IoSnafu {})?;

            // Bash completions
            let mut f =
                fs::File::create("extra/completions/stackablectl.bash").context(IoSnafu {})?;
            generate(Shell::Bash, &mut cmd, name.clone(), &mut f);

            // Fish completions
            let mut f =
                fs::File::create("extra/completions/stackablectl.fish").context(IoSnafu {})?;
            generate(Shell::Fish, &mut cmd, name.clone(), &mut f);

            // ZSH completions
            let mut f = fs::File::create("extra/completions/_stackablectl").context(IoSnafu {})?;
            generate(Shell::Zsh, &mut cmd, name, &mut f);
        }
        XtaskCli::GenOpenapi => {
            let openapi_json = ApiDoc::openapi().to_json().context(SerializeOpenApiSnafu)?;
            let mut codegen = Command::new("pnpm")
                .args(["--filter", "web-ui", "run", "openapi-codegen"])
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
            if !status.success() {
                return ImportOpenapiSchemaSnafu {
                    error_code: status.code(),
                }
                .fail();
            }
        }
    }

    Ok(())
}
