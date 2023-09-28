use std::{
    io::Write,
    process::{Command, Stdio},
};

use snafu::{ensure, ResultExt, Snafu};
use stackable_cockpitd::api_doc::openapi;

#[derive(Debug, Snafu)]
pub enum GenOpenapiError {
    #[snafu(display("error serializing openapi"))]
    SerializeOpenApi { source: serde_json::Error },

    #[snafu(display("error running importing openapi schema importer"))]
    ImportOpenapiSchemaRun { source: std::io::Error },

    #[snafu(display("openapi schema importer failed with error code {error_code:?}"))]
    ImportOpenapiSchema { error_code: Option<i32> },

    #[snafu(display("error writing openapi schema into importer"))]
    WriteOpenapiSchema { source: std::io::Error },
}

pub fn generate() -> Result<(), GenOpenapiError> {
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

    Ok(())
}
