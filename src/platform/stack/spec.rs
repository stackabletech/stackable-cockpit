use serde::{Deserialize, Serialize};
use tracing::{info, instrument};

#[cfg(feature = "openapi")]
use utoipa::ToSchema;

use crate::{
    common::ManifestSpec,
    platform::{demo::DemoParameter, release::ReleaseList, stack::StackError},
    utils::params::{IntoParameters, Parameter, RawParameter, RawParameterParseError},
};

pub type RawStackParameterParseError = RawParameterParseError;
pub type RawStackParameter = RawParameter;
pub type StackParameter = Parameter;

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

        let release = release_list
            .get(&self.release)
            .ok_or(StackError::NoSuchStack)?;

        release.install(&self.operators, &[]);

        todo!()
    }

    #[instrument(skip_all)]
    pub fn install_stack_manifests(&self, parameters: &[String]) -> Result<(), StackError> {
        info!("Installing stack manifests");
        let parameters = parameters.to_owned().into_params(&self.parameters)?;

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
