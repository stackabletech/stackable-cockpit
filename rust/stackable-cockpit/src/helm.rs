use std::fmt::Display;

use indicatif::ProgressStyle;
use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};
use tokio::task::block_in_place;
use tracing::{Span, debug, error, info, instrument};
use tracing_indicatif::span_ext::IndicatifSpanExt;
use url::Url;

use crate::{
    constants::{HELM_DEFAULT_CHART_VERSION, HELM_REPO_INDEX_FILE},
    utils::chartsource::ChartSourceMetadata,
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Release {
    pub name: String,
    pub version: String,
    pub namespace: String,
    pub status: String,
    pub last_updated: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chart {
    pub release_name: String,
    pub name: String,
    pub repo: ChartRepo,
    pub version: String,
    pub options: serde_yaml::Value,
}

#[derive(Debug, Deserialize)]
pub struct ChartRepo {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("failed to parse URL"))]
    UrlParse { source: url::ParseError },

    #[snafu(display("failed to deserialize JSON data"))]
    DeserializeJson { source: serde_json::Error },

    #[snafu(display("failed to deserialize YAML data"))]
    DeserializeYaml { source: serde_yaml::Error },

    #[snafu(display("failed to retrieve remote content"))]
    FetchRemoteContent { source: reqwest::Error },

    #[snafu(display("failed to add Helm repo ({error})"))]
    AddRepo { error: String },

    #[snafu(display("failed to list Helm releases ({error})"))]
    ListReleases { error: String },

    #[snafu(display("failed to install Helm release"))]
    InstallRelease { source: InstallReleaseError },

    #[snafu(display("failed to uninstall Helm release ({error})"))]
    UninstallRelease { error: String },
}

#[derive(Debug, Snafu)]
pub enum InstallReleaseError {
    /// This error indicates that the Helm release was not found, instead of
    /// `check_release_exists` returning true.
    #[snafu(display("failed to find release {name}"))]
    NoSuchRelease { name: String },

    /// This error indicates that the Helm release is already installed at a
    /// different version than requested. Installation is skipped. Existing
    /// releases should be uninstalled with 'stackablectl op un \<NAME\>'.
    #[snafu(display(
        "release {name} ({current_version}) already installed, skipping requested version {requested_version}"
    ))]
    ReleaseAlreadyInstalled {
        name: String,
        current_version: String,
        requested_version: String,
    },

    /// This error indicates that there was an Helm error. The error it self
    /// is not typed, as the error is a plain string coming directly from the
    /// FFI bindings.
    #[snafu(display("helm FFI library call failed ({error})"))]
    HelmWrapper { error: String },
}

#[derive(Debug)]
pub enum InstallReleaseStatus {
    /// Indicates that a release is already installed with a different version
    /// than requested.
    ReleaseAlreadyInstalledWithVersion {
        release_name: String,
        current_version: String,
        requested_version: String,
    },

    /// Indicates that a release is already installed, but no specific version
    /// was requested.
    ReleaseAlreadyInstalledUnspecified {
        release_name: String,
        current_version: String,
    },

    /// Indicates that the release was installed successfully.
    Installed(String),
}

impl Display for InstallReleaseStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstallReleaseStatus::ReleaseAlreadyInstalledWithVersion {
                release_name,
                current_version,
                requested_version,
            } => {
                write!(
                    f,
                    "The release {} ({}) is already installed (requested {}), skipping.",
                    release_name, current_version, requested_version
                )
            }
            InstallReleaseStatus::ReleaseAlreadyInstalledUnspecified {
                release_name,
                current_version,
            } => {
                write!(
                    f,
                    "The release {} ({}) is already installed and no specific version was requested, skipping.",
                    release_name, current_version
                )
            }
            InstallReleaseStatus::Installed(release_name) => {
                write!(
                    f,
                    "The release {} was successfully installed.",
                    release_name
                )
            }
        }
    }
}

#[derive(Debug)]
pub enum UninstallReleaseStatus {
    NotInstalled(String),
    Uninstalled(String),
}

impl Display for UninstallReleaseStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UninstallReleaseStatus::NotInstalled(release_name) => {
                write!(
                    f,
                    "The release {} is not installed, skipping.",
                    release_name
                )
            }
            UninstallReleaseStatus::Uninstalled(release_name) => {
                write!(
                    f,
                    "The release {} was successfully uninstalled.",
                    release_name
                )
            }
        }
    }
}

pub struct ChartVersion<'a> {
    pub chart_source: &'a str,
    pub chart_name: &'a str,
    pub chart_version: Option<&'a str>,
}

/// Installs a Helm release from a repo or registry.
///
/// This function expects the fully qualified Helm release name. In case of our
/// operators this is: `<PRODUCT_NAME>-operator`.
#[instrument(skip(values_yaml), fields(with_values = values_yaml.is_some()))]
pub fn install_release_from_repo_or_registry(
    release_name: &str,
    ChartVersion {
        chart_source,
        chart_name,
        chart_version,
    }: ChartVersion,
    values_yaml: Option<&str>,
    namespace: &str,
    suppress_output: bool,
) -> Result<InstallReleaseStatus, Error> {
    Span::current().pb_set_style(&ProgressStyle::with_template("").unwrap());

    // Ideally, each Helm invocation would spawn_blocking instead in/around helm_sys,
    // but that requires a larger refactoring
    block_in_place(|| {
        debug!("Install Helm release from repo");

        if check_release_exists(release_name, namespace)? {
            let release = get_release(release_name, namespace)?.ok_or(Error::InstallRelease {
                source: InstallReleaseError::NoSuchRelease {
                    name: release_name.to_owned(),
                },
            })?;

            let current_version = release.version;

            match chart_version {
                Some(chart_version) => {
                    if chart_version == current_version {
                        return Ok(InstallReleaseStatus::ReleaseAlreadyInstalledWithVersion {
                            requested_version: chart_version.to_string(),
                            release_name: release_name.to_string(),
                            current_version,
                        });
                    } else {
                        return Err(Error::InstallRelease {
                            source: InstallReleaseError::ReleaseAlreadyInstalled {
                                requested_version: chart_version.into(),
                                name: release_name.into(),
                                current_version,
                            },
                        });
                    }
                }
                None => {
                    return Ok(InstallReleaseStatus::ReleaseAlreadyInstalledUnspecified {
                        release_name: release_name.to_string(),
                        current_version,
                    });
                }
            }
        }

        let full_chart_name = format!("{chart_source}/{chart_name}");
        let chart_version = chart_version.unwrap_or(HELM_DEFAULT_CHART_VERSION);

        debug!(
            release_name,
            chart_version, full_chart_name, "Installing Helm release"
        );

        install_release(
            release_name,
            &full_chart_name,
            chart_version,
            values_yaml,
            namespace,
            suppress_output,
        )?;

        Ok(InstallReleaseStatus::Installed(release_name.to_string()))
    })
}

/// Installs a Helm release.
///
/// This function expects the fully qualified Helm release name. In case of our
/// operators this is: `<PRODUCT_NAME>-operator`.
#[instrument(fields(with_values = values_yaml.is_some()))]
fn install_release(
    release_name: &str,
    chart_name: &str,
    chart_version: &str,
    values_yaml: Option<&str>,
    namespace: &str,
    suppress_output: bool,
) -> Result<(), Error> {
    Span::current().pb_set_style(&ProgressStyle::with_template("").unwrap());

    let result = helm_sys::install_helm_release(
        release_name,
        chart_name,
        chart_version,
        values_yaml.unwrap_or(""),
        namespace,
        suppress_output,
    );

    if let Some(error) = helm_sys::to_helm_error(&result) {
        error!(
            "Go wrapper function go_install_helm_release encountered an error: {}",
            error
        );

        return Err(Error::InstallRelease {
            source: InstallReleaseError::HelmWrapper { error },
        });
    }

    Ok(())
}

/// Uninstall a Helm release.
///
/// This function expects the fully qualified Helm release name. In case of our
/// operators this is: `<PRODUCT_NAME>-operator`.
#[instrument]
pub fn uninstall_release(
    release_name: &str,
    namespace: &str,
    suppress_output: bool,
) -> Result<UninstallReleaseStatus, Error> {
    debug!("Uninstall Helm release");
    Span::current().pb_set_style(&ProgressStyle::with_template("").unwrap());

    if check_release_exists(release_name, namespace)? {
        let result = helm_sys::uninstall_helm_release(release_name, namespace, suppress_output);

        if let Some(err) = helm_sys::to_helm_error(&result) {
            error!(
                "Go wrapper function go_uninstall_helm_release encountered an error: {}",
                err
            );

            return Err(Error::UninstallRelease { error: err });
        }

        return Ok(UninstallReleaseStatus::Uninstalled(
            release_name.to_string(),
        ));
    }

    info!(
        "The Helm release {} is not installed, skipping.",
        release_name
    );

    Ok(UninstallReleaseStatus::NotInstalled(
        release_name.to_string(),
    ))
}

/// Returns if a Helm release exists
#[instrument]
pub fn check_release_exists(release_name: &str, namespace: &str) -> Result<bool, Error> {
    debug!("Check if Helm release exists");
    Span::current().pb_set_style(&ProgressStyle::with_template("").unwrap());

    // TODO (Techassi): Handle error
    Ok(helm_sys::check_helm_release_exists(release_name, namespace))
}

/// Returns a list of Helm releases
#[instrument]
pub fn list_releases(namespace: &str) -> Result<Vec<Release>, Error> {
    debug!("List Helm releases");
    Span::current().pb_set_style(&ProgressStyle::with_template("").unwrap());

    let result = helm_sys::list_helm_releases(namespace);

    if let Some(err) = helm_sys::to_helm_error(&result) {
        error!(
            "Go wrapper function go_helm_list_releases encountered an error: {}",
            err
        );

        return Err(Error::ListReleases { error: err });
    }

    serde_json::from_str(&result).context(DeserializeJsonSnafu)
}

/// Returns a single Helm release by `release_name`.
#[instrument]
pub fn get_release(release_name: &str, namespace: &str) -> Result<Option<Release>, Error> {
    debug!("Get Helm release");
    Span::current().pb_set_style(&ProgressStyle::with_template("").unwrap());

    Ok(list_releases(namespace)?
        .into_iter()
        .find(|r| r.name == release_name))
}

/// Adds a Helm repo with `repo_name` and `repo_url`.
#[instrument]
pub fn add_repo(repository_name: &str, repository_url: &str) -> Result<(), Error> {
    debug!("Add Helm repo");
    Span::current().pb_set_style(&ProgressStyle::with_template("").unwrap());

    let result = helm_sys::add_helm_repository(repository_name, repository_url);

    if let Some(err) = helm_sys::to_helm_error(&result) {
        error!(
            "Go wrapper function go_add_helm_repo encountered an error: {}",
            err
        );

        return Err(Error::AddRepo { error: err });
    }

    Ok(())
}

/// Retrieves the Helm index file from the repository URL.
#[instrument(skip_all, fields(%repo_url))]
pub async fn get_helm_index<T>(repo_url: T) -> Result<ChartSourceMetadata, Error>
where
    T: AsRef<str> + std::fmt::Display + std::fmt::Debug,
{
    debug!("Get Helm repo index file");
    Span::current().pb_set_style(&ProgressStyle::with_template("").unwrap());

    let url = Url::parse(repo_url.as_ref()).context(UrlParseSnafu)?;
    let url = url.join(HELM_REPO_INDEX_FILE).context(UrlParseSnafu)?;

    debug!("Using {} to retrieve Helm index file", url);

    // TODO (Techassi): Use the FileTransferClient for that
    let index_file_content = reqwest::get(url)
        .await
        .context(FetchRemoteContentSnafu)?
        .text()
        .await
        .context(FetchRemoteContentSnafu)?;

    serde_yaml::from_str(&index_file_content).context(DeserializeYamlSnafu)
}
