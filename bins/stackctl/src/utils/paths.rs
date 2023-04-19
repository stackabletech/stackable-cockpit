use std::{convert::Infallible, path::PathBuf};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum PathParseError {
    #[error("parse error")]
    ParseError(#[from] Infallible),
}

/// Converts a string into zero or more paths
pub fn string_to_paths<T>(input: T) -> Result<Vec<PathBuf>, PathParseError>
where
    T: AsRef<str>,
{
    let input = input.as_ref();

    // Fast path, no quoted paths (with spaces in file or directory names)
    if !input.contains(['"', '\'']) {
        let paths = input
            .split(" ")
            .map(|p| p.parse())
            .collect::<Result<Vec<PathBuf>, _>>()?;

        return Ok(paths);
    }

    // TODO (Techassi): Slow path, the string contains quoted paths
    todo!()
}
