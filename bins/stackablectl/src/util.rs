use snafu::Snafu;
use stackable::constants::{HELM_REPO_NAME_DEV, HELM_REPO_NAME_STABLE, HELM_REPO_NAME_TEST};

use crate::constants::{HELM_REPO_URL_DEV, HELM_REPO_URL_STABLE, HELM_REPO_URL_TEST};

#[derive(Debug, Snafu)]
#[snafu(display("Invalid Helm repo name ({name}), cannot resolve to repo URL"))]
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

pub fn pluralize<T>(input: T, size: usize) -> String
where
    T: AsRef<str>,
{
    let input = input.as_ref();
    let suffix = if !input.ends_with("s") && size != 1 {
        "s"
    } else {
        ""
    };

    format!("{}{}", input, suffix)
}
