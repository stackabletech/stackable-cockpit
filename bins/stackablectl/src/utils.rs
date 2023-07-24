use std::env;

use stackable::constants::{HELM_REPO_NAME_DEV, HELM_REPO_NAME_STABLE, HELM_REPO_NAME_TEST};
use thiserror::Error;

use crate::constants::{HELM_REPO_URL_DEV, HELM_REPO_URL_STABLE, HELM_REPO_URL_TEST};

#[derive(Debug, Error)]
#[error("Invalid Helm repo name ({name}), cannot resolve to repo URL")]
pub struct InvalidRepoNameError {
    name: String,
}

/// This returns the Helm repository URL based on the repo name. If the provided
/// repo name is not recognized (invalid), an [`InvalidRepoNameError`] is
/// returned.
pub fn helm_repo_name_to_repo_url<'a, T>(repo_name: T) -> Result<&'a str, InvalidRepoNameError>
where
    T: AsRef<str>,
{
    let repo_name = repo_name.as_ref();

    match repo_name {
        HELM_REPO_NAME_STABLE => Ok(HELM_REPO_URL_STABLE),
        HELM_REPO_NAME_TEST => Ok(HELM_REPO_URL_TEST),
        HELM_REPO_NAME_DEV => Ok(HELM_REPO_URL_DEV),
        _ => Err(InvalidRepoNameError {
            name: repo_name.to_string(),
        }),
    }
}

/// Returns wether the application should use colored output based on the user
/// requested output and the `NO_COLOR` env variable. It currently does not
/// factor in terminal support.
pub fn use_colored_output(use_color: bool) -> bool {
    use_color && env::var_os("NO_COLOR").is_none()
}
