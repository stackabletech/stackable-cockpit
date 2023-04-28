use std::str::{self, Utf8Error};
use std::{collections::HashMap, ffi::CStr, os::raw::c_char};

use serde::Deserialize;
use thiserror::Error;
use tracing::{debug, error, info, instrument};

use crate::constants::HELM_ERROR_PREFIX;

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
    StrUtf8Error(#[from] Utf8Error),

    #[error("json error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("failed to add Helm repo: {0}")]
    AddRepoError(String),

    #[error("failed to list Helm releases: {0}")]
    ListReleasesError(String),

    #[error("failed to install Helm release: {0}")]
    InstallReleaseError(#[from] HelmInstallReleaseError),

    #[error("failed to uninstall Helm release: {0}")]
    UninstallReleaseError(String),
}

#[derive(Debug, Error)]
pub enum HelmInstallReleaseError {
    /// This error indicates that the Helm release was not found, instead of `check_release_exists` returning true.
    #[error("failed to find release {0}")]
    NoSuchRelease(String),

    /// This error indicates that the Helm release is already installed at a different version than requested.
    /// Installation is skipped. Existing releases should be uninstalled with 'stackablectl op un <NAME>'.
    #[error("release {name} ({current_version}) already installed, skipping requested version {requested_version}")]
    ReleaseAlreadyInstalled {
        name: String,
        current_version: String,
        requested_version: String,
    },

    /// This error indicates that there was an Helm error. The error it self is not typed, as the error is a plain
    /// string coming directly from the FFI bindings.
    #[error("helm error: {0}")]
    HelmError(String),
}

#[derive(Debug)]
pub enum HelmInstallReleaseStatus {
    /// Indicates that a release is already installed with a different version than requested.
    ReleaseAlreadyInstalledWithversion {
        current_version: String,
        requested_version: String,
    },

    /// Indicates that a release is already installed, but no specific version was requested.
    ReleaseAlreadyInstalledUnspecified(String),

    /// Indicates that the release was installed successfully.
    Installed,
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
    ) -> *const c_char;
    fn go_uninstall_helm_release(
        release_name: GoString,
        namespace: GoString,
        suppress_output: bool,
    ) -> *const c_char;
    fn go_helm_release_exists(release_name: GoString, namespace: GoString) -> bool;
    fn go_helm_list_releases(namespace: GoString) -> *const c_char;
    fn go_add_helm_repo(name: GoString, url: GoString) -> *const c_char;
}

/// Installs a Helm release
#[instrument]
pub fn install_release_from_repo(
    operator_name: &str,
    release_name: &str,
    repo_name: &str,
    chart_name: &str,
    chart_version: Option<&str>,
    values_yaml: Option<&str>,
    namespace: &str,
    suppress_output: bool,
) -> Result<HelmInstallReleaseStatus, HelmError> {
    debug!("Install Helm release from repo");

    if check_release_exists(release_name, namespace)? {
        let release = get_release(release_name, namespace)?.ok_or(
            HelmInstallReleaseError::NoSuchRelease(release_name.to_string()),
        )?;

        let current_version = release.version;

        match chart_version {
            Some(chart_version) => {
                if chart_version == current_version {
                    return Ok(
                        HelmInstallReleaseStatus::ReleaseAlreadyInstalledWithversion {
                            current_version: current_version.to_string(),
                            requested_version: chart_version.to_string(),
                        },
                    );
                    // return Ok(format!("The release {release_name} ({current_version}) is already installed, skipping."));
                } else {
                    return Err(HelmInstallReleaseError::ReleaseAlreadyInstalled {
                        name: release_name.into(),
                        current_version: current_version.into(),
                        requested_version: chart_version.into(),
                    }
                    .into());
                }
            }
            None => {
                return Ok(
                    HelmInstallReleaseStatus::ReleaseAlreadyInstalledUnspecified(
                        current_version.into(),
                    ),
                )
            }
        }
    }

    let full_chart_name = format!("{repo_name}/{chart_name}");
    let chart_version = chart_version.unwrap_or(">0.0.0-0"); // TODO (Techassi): Move this into a constant

    debug!(
        "Installing Helm release {} ({}) from chart {}",
        repo_name, chart_version, full_chart_name
    );

    install_release(
        release_name,
        &full_chart_name,
        chart_version,
        values_yaml,
        namespace,
        suppress_output,
    )?;

    Ok(HelmInstallReleaseStatus::Installed)
}

fn install_release(
    release_name: &str,
    chart_name: &str,
    chart_version: &str,
    values_yaml: Option<&str>,
    namespace: &str,
    suppress_output: bool,
) -> Result<(), HelmError> {
    let result = unsafe {
        go_install_helm_release(
            release_name.into(),
            chart_name.into(),
            chart_version.into(),
            values_yaml.unwrap_or("").into(),
            namespace.into(),
            suppress_output,
        )
    };

    let result = ptr_to_str(result)?;
    if let Some(err) = to_helm_error(result) {
        error!(
            "Go wrapper function go_install_helm_release encountered an error: {}",
            err
        );

        return Err(HelmInstallReleaseError::HelmError(err).into());
    }

    Ok(())
}

/// Uninstall a Helm release
#[instrument]
pub fn uninstall_release(
    release_name: &str,
    namespace: &str,
    suppress_output: bool,
) -> Result<(), HelmError> {
    debug!("Uninstall Helm release");

    if check_release_exists(release_name, namespace)? {
        let result = unsafe {
            go_uninstall_helm_release(release_name.into(), namespace.into(), suppress_output)
        };

        let result = ptr_to_str(result)?;
        if let Some(err) = to_helm_error(result) {
            error!(
                "Go wrapper function go_uninstall_helm_release encountered an error: {}",
                err
            );

            return Err(HelmError::UninstallReleaseError(err));
        }
    }

    info!(
        "The Helm release {} is not installed, skipping.",
        release_name
    );
    Ok(())
}

/// Returns if a Helm release exists
#[instrument]
pub fn check_release_exists(release_name: &str, namespace: &str) -> Result<bool, HelmError> {
    debug!("Check if Helm release exists");

    // TODO (Techassi): Handle error
    let result = unsafe { go_helm_release_exists(release_name.into(), namespace.into()) };

    Ok(result)
}

/// Returns a list of Helm releases
#[instrument]
pub fn list_releases(namespace: &str) -> Result<Vec<HelmRelease>, HelmError> {
    debug!("List Helm releases");

    let result = unsafe { go_helm_list_releases(namespace.into()) };
    let result = ptr_to_str(result)?;

    if let Some(err) = to_helm_error(result) {
        error!(
            "Go wrapper function go_helm_list_releases encountered an error: {}",
            err
        );

        return Err(HelmError::ListReleasesError(err));
    }

    Ok(serde_json::from_str(result)?)
}

/// Returns a single Helm release by `release_name`.
#[instrument]
pub fn get_release(release_name: &str, namespace: &str) -> Result<Option<HelmRelease>, HelmError> {
    debug!("Get Helm release");

    Ok(list_releases(namespace)?
        .into_iter()
        .find(|r| r.name == release_name))
}

/// Adds a Helm repo with `repo_name` and `repo_url`.
#[instrument]
pub fn add_repo(repo_name: &str, repo_url: &str) -> Result<(), HelmError> {
    debug!("Add Helm repo");

    let result = unsafe { go_add_helm_repo(repo_name.into(), repo_url.into()) };
    let result = ptr_to_str(result)?;

    if let Some(err) = to_helm_error(result) {
        error!(
            "Go wrapper function go_add_helm_repo encountered an error: {}",
            err
        );

        return Err(HelmError::AddRepoError(err));
    }

    Ok(())
}

/// Helper function to convert raw C string pointers to &str.
fn ptr_to_str<'a>(ptr: *const i8) -> Result<&'a str, Utf8Error> {
    let s = unsafe { CStr::from_ptr(ptr) };
    Ok(s.to_str()?)
}

/// Checks if the result string is an error, and if so, returns the error message as a string.
fn to_helm_error(result: &str) -> Option<String> {
    if !result.is_empty() && result.starts_with(HELM_ERROR_PREFIX) {
        return Some(result.replace(HELM_ERROR_PREFIX, "").to_string());
    }

    None
}
