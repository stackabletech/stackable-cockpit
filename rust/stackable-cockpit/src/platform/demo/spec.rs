use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};
use tracing::{debug, instrument, warn};

#[cfg(feature = "openapi")]
use utoipa::ToSchema;

use crate::{
    common::ManifestSpec,
    platform::{
        cluster::{ResourceRequests, ResourceRequestsError},
        release::ReleaseList,
        stack::{StackError, StackList},
    },
    utils::params::{Parameter, RawParameter, RawParameterParseError},
    xfer::FileTransferClient,
};

pub type RawDemoParameterParseError = RawParameterParseError;
pub type RawDemoParameter = RawParameter;
pub type DemoParameter = Parameter;

/// This struct describes a demo with the v2 spec
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct DemoSpecV2 {
    /// A short description of the demo
    pub description: String,

    /// An optional link to a documentation page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,

    /// Supported namespaces this demo can run in. An empty list indicates that
    /// the demo can run in any namespace.
    #[serde(default)]
    pub supported_namespaces: Vec<String>,

    /// The name of the underlying stack
    #[serde(rename = "stackableStack")]
    pub stack: String,

    /// A variable number of labels (tags)
    #[serde(default)]
    pub labels: Vec<String>,

    /// A variable number of Helm or YAML manifests
    #[serde(default)]
    pub manifests: Vec<ManifestSpec>,

    /// The resource requests the demo imposes on a Kubernetes cluster
    pub resource_requests: Option<ResourceRequests>,

    /// A variable number of supported parameters
    #[serde(default)]
    pub parameters: Vec<Parameter>,
}

#[derive(Debug, Snafu)]
pub enum DemoError {
    #[snafu(display("no stack named {name}"))]
    NoSuchStack { name: String },

    #[snafu(display("failed to install stack"))]
    StackError { source: StackError },

    #[snafu(display("demo resource requests error"), context(false))]
    DemoResourceRequestsError { source: ResourceRequestsError },

    #[snafu(display("cannot install demo in namespace '{requested}', only '{}' supported", supported.join(", ")))]
    UnsupportedNamespace {
        requested: String,
        supported: Vec<String>,
    },
}

impl DemoSpecV2 {
    /// Checks if the prerequisites to run this demo are met. These checks
    /// include:
    ///
    /// - Does the demo support to be installed in the requested namespace?
    /// - Does the cluster have enough resources available to run this demo?
    #[instrument(skip_all)]
    pub async fn check_prerequisites(&self, product_namespace: &str) -> Result<(), DemoError> {
        debug!("Checking prerequisites before installing demo");

        // Returns an error if the demo doesn't support to be installed in the
        // requested namespace
        if !self.supports_namespace(product_namespace) {
            return Err(DemoError::UnsupportedNamespace {
                requested: product_namespace.to_string(),
                supported: self.supported_namespaces.clone(),
            });
        }

        // Checks if the available cluster resources are sufficient to deploy
        // the demo.
        if let Some(resource_requests) = &self.resource_requests {
            if let Err(err) = resource_requests.validate_cluster_size("demo").await {
                match err {
                    ResourceRequestsError::ValidationErrors { errors } => {
                        for error in errors {
                            warn!("{error}");
                        }
                    }
                    err => return Err(err.into()),
                }
            }
        }

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn install(
        &self,
        stack_list: StackList,
        release_list: ReleaseList,
        operator_namespace: &str,
        product_namespace: &str,
        stack_parameters: &[String],
        demo_parameters: &[String],
        transfer_client: &FileTransferClient,
        skip_release: bool,
    ) -> Result<(), DemoError> {
        // Get the stack spec based on the name defined in the demo spec
        let stack_spec = stack_list.get(&self.stack).ok_or(DemoError::NoSuchStack {
            name: self.stack.clone(),
        })?;

        // Check prerequisites
        stack_spec
            .check_prerequisites(product_namespace)
            .await
            .context(StackSnafu)?;
        self.check_prerequisites(product_namespace).await?;

        // Install release if not opted out
        if !skip_release {
            stack_spec
                .install_release(release_list, operator_namespace, product_namespace)
                .await
                .context(StackSnafu)?;
        }

        // Install stack
        stack_spec
            .install_stack_manifests(stack_parameters, product_namespace, transfer_client)
            .await
            .context(StackSnafu)?;

        // Install demo manifests
        stack_spec
            .install_demo_manifests(
                &self.manifests,
                &self.parameters,
                demo_parameters,
                product_namespace,
                transfer_client,
            )
            .await
            .context(StackSnafu)?;

        Ok(())
    }

    fn supports_namespace(&self, namespace: impl Into<String>) -> bool {
        self.supported_namespaces.is_empty()
            || self.supported_namespaces.contains(&namespace.into())
    }
}
