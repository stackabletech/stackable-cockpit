use std::{collections::HashMap, path::PathBuf};

use serde::Deserialize;
use snafu::{ResultExt, Snafu};
use tera::{Context, Tera};
use tokio::{fs, io};

#[derive(Debug, Snafu)]
pub enum LocalReadError {
    #[snafu(display("io error: {source}"))]
    IoError { source: io::Error },

    #[snafu(display("templating error: {source}"))]
    TemplatingError { source: tera::Error },

    #[snafu(display("yaml parse error: {source}"))]
    YamlError { source: serde_yaml::Error },
}

pub async fn read_plain_data_from_file(path: PathBuf) -> Result<String, LocalReadError> {
    fs::read_to_string(path).await.context(IoSnafu {})
}

pub async fn read_plain_data_from_file_with_templating(
    path: PathBuf,
    parameters: &HashMap<String, String>,
) -> Result<String, LocalReadError> {
    let content = read_plain_data_from_file(path).await?;

    // Create templating context
    let mut context = Context::new();

    // Fill context with parameters
    for (name, value) in parameters {
        context.insert(name, value)
    }

    // Render template using a one-off function
    Tera::one_off(&content, &context, true).context(TemplatingSnafu)
}

/// Reads YAML data from a local file at `path` and deserializes it into type
/// `T`. A [`LocalReadError`] is returned when the file cannot be read or
/// deserialization failed.
pub async fn read_yaml_data_from_file<T>(path: PathBuf) -> Result<T, LocalReadError>
where
    T: for<'a> Deserialize<'a> + Sized,
{
    let content = read_plain_data_from_file(path).await?;
    let data = serde_yaml::from_str(&content).context(YamlSnafu {})?;

    Ok(data)
}

/// Reads YAML data from a local file at `path` and deserializes it into type
/// `T`. It also inserts parameter values based on templating expressions. The
/// parameters are passed into this function as a [`HashMap<String, String>`].
/// A [`LocalReadError`] is returned when the file cannot be read,
/// deserialization failed or the templating resulted in an error.
pub async fn read_yaml_data_from_file_with_templating<T>(
    path: PathBuf,
    parameters: &HashMap<String, String>,
) -> Result<T, LocalReadError>
where
    T: for<'a> Deserialize<'a>,
{
    let content = read_plain_data_from_file(path).await?;

    // Create templating context
    let mut context = Context::new();

    // Fill context with parameters
    for (name, value) in parameters {
        context.insert(name, value)
    }

    // Render template using a one-off function
    let result = Tera::one_off(&content, &context, true).context(TemplatingSnafu)?;
    serde_yaml::from_str(&result).context(YamlSnafu {})
}
