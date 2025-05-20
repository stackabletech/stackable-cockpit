use indicatif::ProgressStyle;
use serde::{Deserialize, Serialize};
use snafu::{OptionExt, ResultExt, Snafu};
use tracing::{Span, debug, info, instrument, log::warn};
use tracing_indicatif::span_ext::IndicatifSpanExt as _;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

use crate::{
    common::manifest::ManifestSpec,
    platform::{
        cluster::{ResourceRequests, ResourceRequestsError},
        manifests::{self, InstallManifestsExt},
        namespace,
        operator::ChartSourceType,
        release,
        stack::StackInstallParameters,
    },
    utils::{
        k8s::Client,
        params::{
            IntoParameters, IntoParametersError, Parameter, RawParameter, RawParameterParseError,
        },
    },
    xfer,
};

pub type RawStackParameterParseError = RawParameterParseError;
pub type RawStackParameter = RawParameter;
pub type StackParameter = Parameter;

#[derive(Debug, Snafu)]
pub enum Error {
    /// This error indicates that parsing a string into stack / demo parameters
    /// failed.
    #[snafu(display("failed to parse demo / stack parameters"))]
    ParseParameters { source: IntoParametersError },

    /// This error indicates that the requested release doesn't exist in the
    /// loaded list of releases.
    #[snafu(display("no release named {name:?}"))]
    NoSuchRelease { name: String },

    /// This error indicates that the release failed to install.
    #[snafu(display("failed to install release"))]
    InstallRelease { source: release::Error },

    #[snafu(display("stack resource requests error"), context(false))]
    StackResourceRequests { source: ResourceRequestsError },

    /// This error indicates that the stack doesn't support being installed in
    /// the provided namespace.
    #[snafu(display("unable install stack in namespace {requested:?}, only '{}' supported", supported.join(", ")))]
    UnsupportedNamespace {
        requested: String,
        supported: Vec<String>,
    },

    #[snafu(display("failed to create namespace {namespace:?}"))]
    CreateNamespace {
        source: namespace::Error,
        namespace: String,
    },

    #[snafu(display("failed to install stack manifests"))]
    InstallManifests { source: manifests::Error },
}

/// This struct describes a stack with the v2 spec
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct StackSpec {
    /// A short description of the demo
    pub description: String,

    /// The release used by the stack, e.g. 23.4
    #[serde(rename = "stackableRelease")]
    pub release: String,

    /// Supported namespaces this stack can run in. An empty list indicates that
    /// the stack can run in any namespace.
    #[serde(default)]
    pub supported_namespaces: Vec<String>,

    /// A variable number of operators
    #[serde(rename = "stackableOperators")]
    pub operators: Vec<String>,

    /// A variable number of labels (tags)
    #[serde(default)]
    pub labels: Vec<String>,

    /// A variable number of Helm or YAML manifests
    #[serde(default)]
    pub manifests: Vec<ManifestSpec>,

    /// The resource requests the stack imposes on a Kubernetes cluster
    pub resource_requests: Option<ResourceRequests>,

    /// A variable number of supported parameters
    #[serde(default)]
    pub parameters: Vec<StackParameter>,
}

impl InstallManifestsExt for StackSpec {}

impl StackSpec {
    /// Checks if the prerequisites to run this stack are met. These checks
    /// include:
    ///
    /// - Does the stack support to be installed in the requested namespace?
    /// - Does the cluster have enough resources available to run this stack?
    #[instrument(skip_all)]
    pub async fn check_prerequisites(&self, client: &Client, namespace: &str) -> Result<(), Error> {
        debug!("Checking prerequisites before installing stack");

        // Returns an error if the stack doesn't support to be installed in the
        // requested product namespace. When installing a demo, this check is
        // already done on the demo spec level, however we still need to check
        // here, as stacks can be installed on their own.
        if !self.supports_namespace(namespace) {
            return Err(Error::UnsupportedNamespace {
                supported: self.supported_namespaces.clone(),
                requested: namespace.to_owned(),
            });
        }

        // Checks if the available cluster resources are sufficient to deploy
        // the demo.
        if let Some(resource_requests) = &self.resource_requests {
            if let Err(err) = resource_requests
                .validate_cluster_size(client, "stack")
                .await
            {
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

    // TODO (Techassi): Can we get rid of the release list and just use the release spec instead
    #[instrument(skip_all, fields(
        stack_name = %install_parameters.stack_name,
        // NOTE (@NickLarsenNZ): Option doesn't impl Display, so we need to call
        // display for the inner type if it exists. Otherwise we gte the Debug
        // impl for the whole Option.
        demo_name = install_parameters.demo_name.as_ref().map(tracing::field::display),
    ))]
    pub async fn install(
        &self,
        release_list: release::ReleaseList,
        install_parameters: StackInstallParameters,
        client: &Client,
        transfer_client: &xfer::Client,
    ) -> Result<(), Error> {
        // First, we check if the prerequisites are met
        self.check_prerequisites(client, &install_parameters.stack_namespace)
            .await?;

        // Second, we install the release if not opted out
        if !install_parameters.skip_release {
            namespace::create_if_needed(client, install_parameters.operator_namespace.clone())
                .await
                .context(CreateNamespaceSnafu {
                    namespace: install_parameters.operator_namespace.clone(),
                })?;

            self.install_release(
                release_list,
                &install_parameters.operator_namespace,
                &install_parameters.stack_namespace,
                &install_parameters.chart_source,
            )
            .await?;
        }

        // Next, create the product namespace if needed
        // TODO (@NickLarsenNZ): Remove clones (update create_if_needed to take a &str)
        namespace::create_if_needed(client, install_parameters.stack_namespace.clone())
            .await
            .context(CreateNamespaceSnafu {
                namespace: install_parameters.stack_namespace.clone(),
            })?;

        // Finally install the stack manifests
        self.prepare_manifests(install_parameters, client, transfer_client)
            .await
    }

    #[instrument(skip_all, fields(release = %self.release, %operator_namespace))]
    pub async fn install_release(
        &self,
        release_list: release::ReleaseList,
        operator_namespace: &str,
        _namespace: &str, // TODO (@NickLarsenNZ): remove this field
        chart_source: &ChartSourceType,
    ) -> Result<(), Error> {
        info!(self.release, "Trying to install release");
        Span::current().pb_set_style(
            &ProgressStyle::with_template("{spinner} Installing operators")
                .expect("valid progress template"),
        );

        // Get the release by name
        let release = release_list
            .get(&self.release)
            .context(NoSuchReleaseSnafu {
                name: self.release.clone(),
            })?;

        // Install the release
        release
            .install(&self.operators, &[], operator_namespace, chart_source)
            .await
            .context(InstallReleaseSnafu)
    }

    #[instrument(skip_all)]
    pub async fn prepare_manifests(
        &self,
        install_params: StackInstallParameters,
        client: &Client,
        transfer_client: &xfer::Client,
    ) -> Result<(), Error> {
        info!("Installing stack manifests");
        Span::current().pb_set_style(
            &ProgressStyle::with_template("{spinner} Installing manifests")
                .expect("valid progress template"),
        );

        let parameters = install_params
            .parameters
            .to_owned()
            .into_params(&self.parameters)
            .context(ParseParametersSnafu)?;

        Self::install_manifests(
            &self.manifests,
            &parameters,
            &install_params.stack_namespace,
            install_params.labels,
            client,
            transfer_client,
        )
        .await
        .context(InstallManifestsSnafu)
    }

    fn supports_namespace(&self, namespace: impl Into<String>) -> bool {
        self.supported_namespaces.is_empty()
            || self.supported_namespaces.contains(&namespace.into())
    }
}
