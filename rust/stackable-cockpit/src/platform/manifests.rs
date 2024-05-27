use std::collections::HashMap;

use snafu::{ResultExt, Snafu};
use stackable_operator::kvp::Labels;
use tracing::{debug, info, instrument};

use crate::{
    common::manifest::ManifestSpec,
    helm,
    utils::{
        k8s::{self, Client},
        path::{IntoPathOrUrl, PathOrUrlParseError},
    },
    xfer::{
        self,
        processor::{Processor, Template, Yaml},
    },
};

#[derive(Debug, Snafu)]
pub enum Error {
    /// This error indicates that parsing a string into a path or URL failed.
    #[snafu(display("failed to parse '{path_or_url}' as path/url"))]
    ParsePathOrUrl {
        source: PathOrUrlParseError,
        path_or_url: String,
    },

    /// This error indicates that receiving remote content failed.
    #[snafu(display("failed to receive remote content"))]
    FileTransfer { source: xfer::Error },

    /// This error indicates that the Helm wrapper failed to add the Helm
    /// repository.
    #[snafu(display("failed to add Helm repository {repo_name}"))]
    AddHelmRepository {
        source: helm::Error,
        repo_name: String,
    },

    /// This error indicates that the Hlm wrapper failed to install the Helm
    /// release.
    #[snafu(display("failed to install Helm release {release_name}"))]
    InstallHelmRelease {
        release_name: String,
        source: helm::Error,
    },

    /// This error indicates that Helm chart options could not be serialized
    /// into YAML.
    #[snafu(display("failed to serialize Helm chart options"))]
    SerializeOptions { source: serde_yaml::Error },

    /// This error indicates that the creation of a kube client failed.
    #[snafu(display("failed to create Kubernetes client"))]
    CreateKubeClient { source: k8s::Error },

    /// This error indicates that the kube client failed to deloy manifests.
    #[snafu(display("failed to deploy manifests using the kube client"))]
    DeployManifest { source: k8s::Error },
}

pub trait InstallManifestsExt {
    // TODO (Techassi): This step shouldn't care about templating the manifests nor fetching them from remote
    #[instrument(skip_all)]
    #[allow(async_fn_in_trait)]
    async fn install_manifests(
        manifests: &[ManifestSpec],
        parameters: &HashMap<String, String>,
        product_namespace: &str,
        labels: Labels,
        client: &Client,
        transfer_client: &xfer::Client,
    ) -> Result<(), Error> {
        debug!("Installing demo / stack manifests");

        for manifest in manifests {
            match manifest {
                ManifestSpec::HelmChart(helm_file) => {
                    debug!("Installing manifest from Helm chart {}", helm_file);

                    // Read Helm chart YAML and apply templating
                    let helm_file = helm_file.into_path_or_url().context(ParsePathOrUrlSnafu {
                        path_or_url: helm_file.clone(),
                    })?;

                    let helm_chart: helm::Chart = transfer_client
                        .get(&helm_file, &Template::new(parameters).then(Yaml::new()))
                        .await
                        .context(FileTransferSnafu)?;

                    info!(
                        "Installing Helm chart {} ({})",
                        helm_chart.name, helm_chart.version
                    );

                    helm::add_repo(&helm_chart.repo.name, &helm_chart.repo.url).context(
                        AddHelmRepositorySnafu {
                            repo_name: helm_chart.repo.name.clone(),
                        },
                    )?;

                    // Serialize chart options to string
                    let values_yaml = serde_yaml::to_string(&helm_chart.options)
                        .context(SerializeOptionsSnafu)?;

                    // Install the Helm chart using the Helm wrapper
                    helm::install_release_from_repo(
                        &helm_chart.release_name,
                        helm::ChartVersion {
                            repo_name: &helm_chart.repo.name,
                            chart_name: &helm_chart.name,
                            chart_version: Some(&helm_chart.version),
                        },
                        Some(&values_yaml),
                        product_namespace,
                        true,
                    )
                    .context(InstallHelmReleaseSnafu {
                        release_name: helm_chart.release_name,
                    })?;
                }
                ManifestSpec::PlainYaml(manifest_file) => {
                    debug!("Installing YAML manifest from {}", manifest_file);

                    // Read YAML manifest and apply templating
                    let path_or_url =
                        manifest_file
                            .into_path_or_url()
                            .context(ParsePathOrUrlSnafu {
                                path_or_url: manifest_file.clone(),
                            })?;

                    let manifests = transfer_client
                        .get(&path_or_url, &Template::new(parameters))
                        .await
                        .context(FileTransferSnafu)?;

                    client
                        .deploy_manifests(&manifests, product_namespace, labels.clone())
                        .await
                        .context(DeployManifestSnafu)?
                }
            }
        }

        Ok(())
    }
}
