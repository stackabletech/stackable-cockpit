use const_format::concatcp;

pub const ENV_KEY_RELEASE_FILES: &str = "STACKABLE_RELEASE_FILES";
pub const ENV_KEY_STACK_FILES: &str = "STACKABLE_STACK_FILES";
pub const ENV_KEY_DEMO_FILES: &str = "STACKABLE_DEMO_FILES";

pub const DEMO_STACK_BASE_URL: &str = "https://raw.githubusercontent.com/stackabletech/demos/main/";
pub const RELEASE_BASE_URL: &str = "https://raw.githubusercontent.com/stackabletech/release/main/";
pub const REPO_BASE_URL: &str = "https://repo.stackable.tech/repository/";

pub const STACK_FILE_URL: &str = concatcp!(DEMO_STACK_BASE_URL, "stacks/stacks-v2.yaml");
pub const DEMO_FILE_URL: &str = concatcp!(DEMO_STACK_BASE_URL, "demos/demos-v2.yaml");
pub const RELEASE_FILE_URL: &str = concatcp!(RELEASE_BASE_URL, "releases.yaml");

pub const HELM_REPO_URL_STABLE: &str = concatcp!(REPO_BASE_URL, "helm-stable/");
pub const HELM_REPO_URL_TEST: &str = concatcp!(REPO_BASE_URL, "helm-test/");
pub const HELM_REPO_URL_DEV: &str = concatcp!(REPO_BASE_URL, "helm-dev/");

pub const USER_DIR_APPLICATION_NAME: &str = "stackablectl";
pub const USER_DIR_ORGANIZATION_NAME: &str = "Stackable";
pub const USER_DIR_QUALIFIER: &str = "tech";
