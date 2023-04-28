use std::str;
use std::{collections::HashMap, ffi::CStr, os::raw::c_char};

use serde::Deserialize;
use thiserror::Error;
use tracing::{debug, error, instrument};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HelmRelease {
    pub name: String,
    pub version: String,
    pub namespace: String,
    pub status: String,
    pub last_updated: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct HelmRepo {
    pub entries: HashMap<String, Vec<HelmRepoEntry>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct HelmRepoEntry {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Error)]
pub enum HelmError {
    #[error("str utf-8 error: {0}")]
    StrUtf8Error(#[from] str::Utf8Error),

    #[error("failed to add Helm repo: {0}")]
    AddRepoError(String),
}

#[repr(C)]
pub struct GoString {
    p: *const u8,
    n: i64,
}

impl From<&str> for GoString {
    fn from(str: &str) -> Self {
        GoString {
            p: str.as_ptr(),
            n: str.len() as i64,
        }
    }
}

extern "C" {
    fn go_install_helm_release(
        release_name: GoString,
        chart_name: GoString,
        chart_version: GoString,
        values_yaml: GoString,
        namespace: GoString,
        suppress_output: bool,
    );
    fn go_uninstall_helm_release(
        release_name: GoString,
        namespace: GoString,
        suppress_output: bool,
    );
    fn go_helm_release_exists(release_name: GoString, namespace: GoString) -> bool;
    fn go_helm_list_releases(namespace: GoString) -> *const c_char;
    fn go_add_helm_repo(name: GoString, url: GoString) -> *const c_char;
}

/// Installs a Helm release
pub fn install_release() -> Result<(), HelmError> {
    todo!()
}

/// Uninstall a Helm release
pub fn uninstall_release() -> Result<(), HelmError> {
    todo!()
}

/// Resturns if a Helm release exists
pub fn check_release_exists() -> Result<bool, HelmError> {
    todo!()
}

/// Returns a list of Helm releases
#[instrument]
pub fn list_releases(namespace: &str) -> Result<Vec<HelmRelease>, HelmError> {
    debug!("List Helm releases");

    let result = unsafe { go_helm_list_releases(GoString::from(namespace)) };
    let result = unsafe { CStr::from_ptr(result) };

    todo!()
}

/// Adds a Helm repo with `repo_name` and `repo_url`.
#[instrument]
pub fn add_repo(repo_name: &str, repo_url: &str) -> Result<(), HelmError> {
    debug!("Add Helm repo");

    let result = unsafe { go_add_helm_repo(GoString::from(repo_name), GoString::from(repo_url)) };
    let result = unsafe { CStr::from_ptr(result) };

    // If there is an error, the Go function returns a non-empty string
    let err = result.to_str()?;
    if !err.is_empty() {
        error!(
            "Go wrapper function go_add_helm_repo encountered an error: {}",
            err
        );
        return Err(HelmError::AddRepoError(err.to_string()));
    }

    Ok(())
}
