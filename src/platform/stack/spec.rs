use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};
use tracing::{debug, info, instrument};

#[cfg(feature = "openapi")]
use utoipa::ToSchema;

use crate::{
    common::ManifestSpec,
    helm::{self, HelmChart, HelmError},
    kube::{self, KubeClientError},
    platform::{
        demo::DemoParameter,
        release::{ReleaseInstallError, ReleaseList},
    },
    utils::{
        params::{
            IntoParameters, IntoParametersError, Parameter, RawParameter, RawParameterParseError,
        },
        read::{read_yaml_data_with_templating, TemplatedReadError},
    },
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

    /// This error indicates that reading YAML data and applying templating
    /// failed.
    #[snafu(display("templated read error: {source}"))]
    TemplatedReadError { source: TemplatedReadError },

    /// This error indicates the Helm wrapper encountered an error.
    #[snafu(display("Helm error: {source}"))]
    HelmError { source: HelmError },

    #[snafu(display("kube error: {source}"))]
    KubeError { source: KubeClientError },

    /// This error indicates a YAML error occurred.
    #[snafu(display("yaml error: {source}"))]
    YamlError { source: serde_yaml::Error },
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
            .context(ReleaseInstallSnafu)?;

        Ok(())
    }

    #[instrument(skip_all)]
    pub async fn install_stack_manifests(
        &self,
        parameters: &[String],
        namespace: &str,
    ) -> Result<(), StackError> {
        info!("Installing stack manifests");

        let parameters = parameters
            .to_owned()
            .into_params(&self.parameters)
            .context(ParameterSnafu)?;

        Self::install_manifests(&self.manifests, &parameters, namespace).await?;
        Ok(())
    }

    #[instrument(skip_all)]
    pub async fn install_demo_manifests(
        &self,
        manifests: &Vec<ManifestSpec>,
        valid_demo_parameters: &[DemoParameter],
        demo_parameters: &[String],
        namespace: &str,
    ) -> Result<(), StackError> {
        info!("Installing demo manifests");

        let parameters = demo_parameters
            .to_owned()
            .into_params(valid_demo_parameters)
            .context(ParameterSnafu)?;

        Self::install_manifests(manifests, &parameters, namespace).await?;
        Ok(())
    }

    /// Installs a list of templated manifests inside a namespace.
    #[instrument(skip_all)]
    async fn install_manifests(
        manifests: &Vec<ManifestSpec>,
        parameters: &HashMap<String, String>,
        namespace: &str,
    ) -> Result<(), StackError> {
        debug!("Installing demo / stack manifests");

        for manifest in manifests {
            match manifest {
                ManifestSpec::HelmChart(helm_file) => {
                    debug!("Installing manifest from Helm chart {}", helm_file);

                    // Read Helm chart YAML and apply templating
                    let helm_chart = read_yaml_data_with_templating(helm_file, parameters)
                        .await
                        .context(TemplatedReadSnafu)?;
                    let helm_chart: HelmChart =
                        serde_yaml::from_str(&helm_chart).context(YamlSnafu)?;

                    info!(
                        "Installing Helm chart {} ({})",
                        helm_chart.name, helm_chart.version
                    );

                    helm::add_repo(&helm_chart.repo.name, &helm_chart.repo.url)
                        .context(HelmSnafu)?;

                    // Serialize chart options to string
                    let values_yaml =
                        serde_yaml::to_string(&helm_chart.options).context(YamlSnafu)?;

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
                    .context(HelmSnafu)?;
                }
                ManifestSpec::PlainYaml(path_or_url) => {
                    debug!("Installing YAML manifest from {}", path_or_url);

                    // Read YAML manifest and apply templating
                    let manifests = read_yaml_data_with_templating(path_or_url, parameters)
                        .await
                        .context(TemplatedReadSnafu)?;

                    let kube_client = kube::KubeClient::new().await.context(KubeSnafu)?;
                    kube_client
                        .deploy_manifests(&manifests, namespace)
                        .await
                        .context(KubeSnafu)?
                }
            }
        }

        Ok(())
    }
}
