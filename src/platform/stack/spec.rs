use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};
use tracing::{info, instrument};

#[cfg(feature = "openapi")]
use utoipa::ToSchema;

use crate::{
    common::ManifestSpec,
    platform::{
        demo::DemoParameter,
        release::{ReleaseInstallError, ReleaseList},
    },
    utils::{
        params::{
            IntoParameters, IntoParametersError, Parameter, RawParameter, RawParameterParseError,
        },
        path::PathOrUrl,
        read::read_yaml_data_with_templating,
    },
};

pub type RawStackParameterParseError = RawParameterParseError;
pub type RawStackParameter = RawParameter;
pub type StackParameter = Parameter;

#[derive(Debug, Snafu)]
pub enum StackError {
    #[snafu(display("parameter parse error: {source}"))]
    ParameterError { source: IntoParametersError },

    #[snafu(display("no such stack"))]
    NoSuchStack,

    #[snafu(display("release install error: {source}"))]
    ReleaseInstallError { source: ReleaseInstallError },
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
    pub fn install(&self, release_list: ReleaseList) -> Result<(), StackError> {
        info!("Installing stack");

        // Get the release by name
        let release = release_list
            .get(&self.release)
            .ok_or(StackError::NoSuchStack)?;

        // Install the release
        release
            .install(&self.operators, &[])
            .context(ReleaseInstallSnafu {})?;

        todo!()
    }

    #[instrument(skip_all)]
    pub fn install_stack_manifests(&self, parameters: &[String]) -> Result<(), StackError> {
        info!("Installing stack manifests");

        let parameters = parameters
            .to_owned()
            .into_params(&self.parameters)
            .context(ParameterSnafu {})?;

        for manifest in &self.manifests {
            match manifest {
                ManifestSpec::HelmChart(helm_file) => {
                    let helm_chart = read_yaml_data_with_templating(helm_file, &parameters);
                }
                ManifestSpec::PlainYaml(_) => todo!(),
            }
        }

        todo!()
    }

    #[instrument(skip_all)]
    pub fn install_demo_manifests(
        &self,
        valid_demo_parameters: &[DemoParameter],
        demo_parameters: &[String],
    ) -> Result<(), StackError> {
        info!("Installing demo manifests");

        todo!()
    }
}
