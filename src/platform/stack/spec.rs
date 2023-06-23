use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};
use tracing::{info, instrument};

#[cfg(feature = "openapi")]
use utoipa::ToSchema;

use crate::{
    common::ManifestSpec,
    helm::{self, HelmChart, HelmError},
    kube::{self, KubeError},
    platform::{
        demo::DemoParameter,
        release::{ReleaseInstallError, ReleaseList},
    },
    utils::{
        params::{
            IntoParameters, IntoParametersError, Parameter, RawParameter, RawParameterParseError,
        },
        path::{IntoPathOrUrl, PathOrUrl, PathOrUrlParseError},
        read::{
            read_plain_data_from_file_with_templating, read_yaml_data_from_file_with_templating,
            LocalReadError,
        },
    },
    xfer::{TransferClient, TransferError},
};

pub type RawStackParameterParseError = RawParameterParseError;
pub type RawStackParameter = RawParameter;
pub type StackParameter = Parameter;

/// This error indicates that the stack, the stack manifests or the demo
/// manifests failed to install.
#[derive(Debug, Snafu)]
pub enum StackError {
    /// This error indicates that parsing a string into stack / demo parameters
    /// failed.
    #[snafu(display("parameter parse error: {source}"))]
    ParameterError { source: IntoParametersError },

    /// This error indicates that the requested stack doesn't exist in the
    /// loaded list of stacks.
    #[snafu(display("no such stack"))]
    NoSuchStack,

    /// This error indicates that the release failed to install.
    #[snafu(display("release install error: {source}"))]
    ReleaseInstallError { source: ReleaseInstallError },

    /// This error indicates the Helm wrapper encountered an error.
    #[snafu(display("Helm error: {source}"))]
    HelmError { source: HelmError },

    #[snafu(display("kube error: {source}"))]
    KubeError { source: KubeError },

    /// This error indicates a YAML error occurred.
    #[snafu(display("yaml error: {source}"))]
    YamlError { source: serde_yaml::Error },

    #[snafu(display("path or url parse error"))]
    PathOrUrlParseError { source: PathOrUrlParseError },

    #[snafu(display("transfer error"))]
    TransferError { source: TransferError },

    #[snafu(display("local read error"))]
    LocalReadError { source: LocalReadError },
}

/// This struct describes a stack with the v2 spec
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct StackSpecV2 {
    /// A short description of the demo
    pub description: String,

    /// The release used by the stack, e.g. 23.4
    #[serde(rename = "stackableRelease")]
    pub release: String,

    /// A variable number of operators
    #[serde(rename = "stackableOperators")]
    pub operators: Vec<String>,

    /// A variable number of labels (tags)
    #[serde(default)]
    pub labels: Vec<String>,

    /// A variable number of Helm or YAML manifests
    #[serde(default)]
    pub manifests: Vec<ManifestSpec>,

    /// A variable number of supported parameters
    #[serde(default)]
    pub parameters: Vec<StackParameter>,
}

impl StackSpecV2 {
    #[instrument(skip_all)]
    pub fn install(&self, release_list: ReleaseList, namespace: &str) -> Result<(), StackError> {
        info!("Installing stack");

        // Get the release by name
        let release = release_list
            .get(&self.release)
            .ok_or(StackError::NoSuchStack)?;

        // Install the release
        release
            .install(&self.operators, &[], namespace)
            .context(ReleaseInstallSnafu {})?;

        todo!()
    }

    #[instrument(skip_all)]
    pub async fn install_stack_manifests(
        &self,
        parameters: &[String],
        namespace: &str,
        transfer_client: &TransferClient,
    ) -> Result<(), StackError> {
        info!("Installing stack manifests");

        let parameters = parameters
            .to_owned()
            .into_params(&self.parameters)
            .context(ParameterSnafu {})?;

        Self::install_manifests(&self.manifests, &parameters, namespace, transfer_client).await?;
        Ok(())
    }

    #[instrument(skip_all)]
    pub async fn install_demo_manifests(
        &self,
        manifests: &Vec<ManifestSpec>,
        valid_demo_parameters: &[DemoParameter],
        demo_parameters: &[String],
        namespace: &str,
        transfer_client: &TransferClient,
    ) -> Result<(), StackError> {
        info!("Installing demo manifests");

        let parameters = demo_parameters
            .to_owned()
            .into_params(valid_demo_parameters)
            .context(ParameterSnafu {})?;

        Self::install_manifests(manifests, &parameters, namespace, transfer_client).await?;
        Ok(())
    }

    /// Installs a list of templated manifests inside a namespace.
    #[instrument(skip_all)]
    async fn install_manifests(
        manifests: &Vec<ManifestSpec>,
        parameters: &HashMap<String, String>,
        namespace: &str,
        transfer_client: &TransferClient,
    ) -> Result<(), StackError> {
        for manifest in manifests {
            match manifest {
                ManifestSpec::HelmChart(helm_file) => {
                    // Read Helm chart YAML and apply templating
                    let helm_chart: HelmChart =
                        match helm_file.into_path_or_url().context(PathOrUrlParseSnafu)? {
                            PathOrUrl::Path(path) => {
                                read_yaml_data_from_file_with_templating(path, parameters)
                                    .await
                                    .context(LocalReadSnafu)?
                            }
                            PathOrUrl::Url(url) => transfer_client
                                .get_templated_yaml_data(&url, parameters)
                                .await
                                .context(TransferSnafu)?,
                        };

                    info!(
                        "Installing Helm chart {} ({})",
                        helm_chart.name, helm_chart.version
                    );

                    helm::add_repo(&helm_chart.repo.name, &helm_chart.repo.url)
                        .context(HelmSnafu {})?;

                    // Serialize chart options to string
                    let values_yaml =
                        serde_yaml::to_string(&helm_chart.values).context(YamlSnafu {})?;

                    // Install the Helm chart using the Helm wrapper
                    helm::install_release_from_repo(
                        &helm_chart.release_name,
                        &helm_chart.release_name,
                        helm::ChartVersion {
                            repo_name: &helm_chart.repo.name,
                            chart_name: &helm_chart.name,
                            chart_version: Some(&helm_chart.version),
                        },
                        Some(&values_yaml),
                        namespace,
                        false,
                    )
                    .context(HelmSnafu {})?;
                }
                ManifestSpec::PlainYaml(path_or_url) => {
                    info!("Installing YAML manifest from {}", path_or_url);

                    // Read YAML manifest and apply templating
                    let manifests = match path_or_url
                        .into_path_or_url()
                        .context(PathOrUrlParseSnafu)?
                    {
                        PathOrUrl::Path(path) => {
                            read_plain_data_from_file_with_templating(path, parameters)
                                .await
                                .context(LocalReadSnafu)?
                        }
                        PathOrUrl::Url(url) => transfer_client
                            .get_templated_yaml_data(&url, parameters)
                            .await
                            .context(TransferSnafu)?,
                    };

                    kube::deploy_manifests(&manifests, namespace)
                        .await
                        .context(KubeSnafu {})?;
                }
            }
        }

        Ok(())
    }
}
