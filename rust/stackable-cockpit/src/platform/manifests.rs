use std::{collections::HashMap, time::Duration};

use indicatif::ProgressStyle;
use snafu::{ResultExt, Snafu};
use stackable_operator::kvp::Labels;
use tracing::{Instrument as _, Span, debug, info, info_span, instrument};
use tracing_indicatif::span_ext::IndicatifSpanExt as _;

use crate::{
    PROGRESS_BAR_STYLE,
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
    #[instrument(skip_all, fields(%namespace, indicatif.pb_show = true))]
    #[allow(async_fn_in_trait)]
    async fn install_manifests(
        manifests: &[ManifestSpec],
        parameters: &HashMap<String, String>,
        namespace: &str,
        labels: Labels,
        client: &Client,
        transfer_client: &xfer::Client,
    ) -> Result<(), Error> {
        debug!("Installing manifests");

        Span::current().pb_set_style(&PROGRESS_BAR_STYLE);
        Span::current().pb_set_length(manifests.len() as u64);

        tokio::time::sleep(Duration::from_secs(2)).await;

        let mut parameters = parameters.clone();
        // We add the NAMESPACE parameter, so that stacks/demos can use that to render e.g. the
        // fqdn service names [which contain the namespace].
        parameters.insert("NAMESPACE".to_owned(), namespace.to_owned());

        for manifest in manifests {
            let span = tracing::warn_span!("install_manifests_iter", indicatif.pb_show = true);
            span.pb_set_style(
                &ProgressStyle::with_template("{span_child_prefix} boo {span_name}").unwrap(),
            );

            let parameters = parameters.clone();
            let labels = labels.clone();
            async move {
                match manifest {
                    ManifestSpec::HelmChart(helm_file) => {
                        debug!(helm_file, "Installing manifest from Helm chart");

                        // Read Helm chart YAML and apply templating
                        let helm_file =
                            helm_file.into_path_or_url().context(ParsePathOrUrlSnafu {
                                path_or_url: helm_file.clone(),
                            })?;

                        let helm_chart: helm::Chart = transfer_client
                            .get(&helm_file, &Template::new(&parameters).then(Yaml::new()))
                            .await
                            .context(FileTransferSnafu)?;

                        info!(helm_chart.name, helm_chart.version, "Installing Helm chart");
                        Span::current().pb_set_message(
                            format!("Installing {name} Helm chart", name = helm_chart.name)
                                .as_str(),
                        );
                        // Span::current().pb_set_style(
                        //     &ProgressStyle::with_template(
                        //         "{span_child_prefix:.bold.dim} {spinner} {span_name}",
                        //     )
                        //     .expect("valid progress template"),
                        // );

                        // Assumption: that all manifest helm charts refer to repos not registries
                        helm::add_repo(&helm_chart.repo.name, &helm_chart.repo.url).context(
                            AddHelmRepositorySnafu {
                                repo_name: helm_chart.repo.name.clone(),
                            },
                        )?;

                        // Serialize chart options to string
                        let values_yaml = serde_yaml::to_string(&helm_chart.options)
                            .context(SerializeOptionsSnafu)?;

                        // Install the Helm chart using the Helm wrapper
                        helm::install_release_from_repo_or_registry(
                            &helm_chart.release_name,
                            helm::ChartVersion {
                                chart_source: &helm_chart.repo.name,
                                chart_name: &helm_chart.name,
                                chart_version: Some(&helm_chart.version),
                            },
                            Some(&values_yaml),
                            namespace,
                            true,
                        )
                        .context(InstallHelmReleaseSnafu {
                            release_name: helm_chart.release_name,
                        })?;
                    }
                    ManifestSpec::PlainYaml(manifest_file) => {
                        debug!(manifest_file, "Installing YAML manifest");
                        // TODO (@NickLarsenNZ): This span already has a style.
                        // Span::current().pb_set_style(
                        //     &ProgressStyle::with_template(
                        //         "{span_child_prefix:.bold.dim} {spinner} {span_name} Installing YAML manifest",
                        //     )
                        //     .expect("valid progress template"),
                        // );

                        // Read YAML manifest and apply templating
                        let path_or_url =
                            manifest_file
                                .into_path_or_url()
                                .context(ParsePathOrUrlSnafu {
                                    path_or_url: manifest_file.clone(),
                                })?;

                        let manifests = transfer_client
                            .get(&path_or_url, &Template::new(&parameters))
                            .await
                            .context(FileTransferSnafu)?;

                        client
                            .deploy_manifests(&manifests, namespace, labels.clone())
                            .await
                            .context(DeployManifestSnafu)?;
                    }
                }

                Ok::<(), Error>(())
            }
            .instrument(span)
            .await?;

            Span::current().pb_inc(1);
        }

        Ok(())
    }
}
