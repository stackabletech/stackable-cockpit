use std::collections::HashMap;

use serde::Deserialize;
use snafu::{OptionExt, ResultExt, Snafu};
use tracing::debug;
use urlencoding::encode;

use crate::{
    constants::{
        HELM_OCI_BASE, HELM_REPO_NAME_DEV, HELM_REPO_NAME_STABLE, HELM_REPO_NAME_TEST,
        OCI_INDEX_PAGE_SIZE,
    },
    utils::chartsource::{ChartSourceEntry, ChartSourceMetadata},
};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("cannot get repositories"))]
    GetRepositories { source: reqwest::Error },

    #[snafu(display("cannot parse repositories"))]
    ParseRepositories { source: reqwest::Error },

    #[snafu(display("cannot get artifacts"))]
    GetArtifacts { source: reqwest::Error },

    #[snafu(display("cannot parse artifacts"))]
    ParseArtifacts { source: reqwest::Error },

    #[snafu(display("unexpected OCI repository name"))]
    UnexpectedOciRepositoryName,
}

/// Identifies an operator-specific root folder in the repository e.g.
/// ```
/// {
///   name: "sdp-charts/airflow-operator"
/// }
/// ```
#[derive(Deserialize, Debug)]
struct OciRepository {
    pub name: String,
}

/// Identifies an image tag e.g.
/// ```
/// {
///   name: "24.11.1-rc1"
/// }
/// ```
#[derive(Deserialize, Debug)]
pub struct Tag {
    pub name: String,
}

/// Identifies an image artifact with its digest and tags e.g.
/// ```
/// {
///   digest: "sha256:e80a4b1e004f90dee0321f817871c4a225369b89efdc17c319595263139364b5",
///   tags: [
///     {
///       name: "0.0.0-pr569"
///     }
///   ])
/// }
/// ```
#[derive(Deserialize, Debug)]
pub struct Artifact {
    pub digest: String,
    pub tags: Option<Vec<Tag>>,
}

pub async fn get_oci_index<'a>() -> Result<HashMap<&'a str, ChartSourceMetadata>, Error> {
    let mut source_index_files: HashMap<&str, ChartSourceMetadata> = HashMap::new();

    // initialize map
    for repo_name in [
        HELM_REPO_NAME_STABLE,
        HELM_REPO_NAME_TEST,
        HELM_REPO_NAME_DEV,
    ] {
        source_index_files.insert(
            repo_name,
            ChartSourceMetadata {
                entries: HashMap::new(),
            },
        );
    }
    let base_url = format!("https://{}/api/v2.0", HELM_OCI_BASE);

    // fetch all operators
    let url = format!(
        "{}/repositories?page_size={}&q=name=~sdp-charts/",
        base_url, 100
    );

    // reuse connections
    let client = reqwest::Client::new();

    let repositories: Vec<OciRepository> = client
        .get(&url)
        .send()
        .await
        .context(GetRepositoriesSnafu)?
        .json()
        .await
        .context(ParseRepositoriesSnafu)?;

    debug!("OCI repos {:?}", repositories);

    for repository in &repositories {
        // fetch all artifacts pro operator
        let (project_name, repository_name) = repository
            .name
            .split_once('/')
            .context(UnexpectedOciRepositoryNameSnafu)?;

        debug!("OCI repo parts {} and {}", project_name, repository_name);

        let mut artifacts = Vec::new();
        let mut page = 1;

        loop {
            let url = format!(
                "{}/projects/{}/repositories/{}/artifacts?page_size={}&page={}",
                base_url,
                encode(project_name),
                encode(repository_name),
                OCI_INDEX_PAGE_SIZE,
                page
            );

            let artifacts_page = client
                .get(url)
                .send()
                .await
                .context(GetArtifactsSnafu)?
                .json::<Vec<Artifact>>()
                .await
                .context(ParseArtifactsSnafu)?;

            let count = artifacts_page.len();
            artifacts.extend(artifacts_page);
            if count < OCI_INDEX_PAGE_SIZE {
                break;
            }
            page += 1;
        }

        for artifact in &artifacts {
            if let Some(release_artifact) =
                artifact.tags.as_ref().and_then(|tags| tags.iter().next())
            {
                let release_version = release_artifact
                    .name
                    .replace("-arm64", "")
                    .replace("-amd64", "");

                debug!(
                    "OCI resolved artifact {}, {}, {}",
                    release_version.to_string(),
                    repository_name.to_string(),
                    release_artifact.name.to_string()
                );

                debug!(
                    "Repo/Artifact/Tag: {:?} / {:?} / {:?}",
                    repository, artifact, release_artifact
                );

                let entry = ChartSourceEntry {
                    name: repository_name.to_string(),
                    version: release_version.to_string(),
                };

                match release_version.as_str() {
                    "0.0.0-dev" => {
                        if let Some(repo) = source_index_files.get_mut(HELM_REPO_NAME_DEV) {
                            repo.entries
                                .entry(repository_name.to_string())
                                .or_default()
                                .push(entry)
                        }
                    }
                    version if version.contains("-pr") => {
                        if let Some(repo) = source_index_files.get_mut(HELM_REPO_NAME_TEST) {
                            repo.entries
                                .entry(repository_name.to_string())
                                .or_default()
                                .push(entry)
                        }
                    }
                    _ => {
                        if let Some(repo) = source_index_files.get_mut(HELM_REPO_NAME_STABLE) {
                            repo.entries
                                .entry(repository_name.to_string())
                                .or_default()
                                .push(entry)
                        }
                    }
                }
            }
        }
    }
    Ok(source_index_files)
}
