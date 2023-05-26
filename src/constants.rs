use std::time::Duration;

pub const DEFAULT_STACKABLE_NAMESPACE: &str = "stackable";
pub const DEFAULT_NAMESPACE: &str = "default";

pub const DEFAULT_LOCAL_CLUSTER_NAME: &str = "stackable-data-platform";

pub const DEFAULT_CACHE_MAX_AGE: Duration = Duration::from_secs(60 * 60); // One hour

pub const HELM_REPO_NAME_STABLE: &str = "stackable-stable";
pub const HELM_REPO_NAME_TEST: &str = "stackable-test";
pub const HELM_REPO_NAME_DEV: &str = "stackable-dev";
pub const HELM_REPO_INDEX_FILE: &str = "index.yaml";

pub const HELM_DEFAULT_CHART_VERSION: &str = ">0.0.0-0";
pub const HELM_ERROR_PREFIX: &str = "ERROR:";
