use std::time::Duration;

pub const REDACTED_PASSWORD: &str = "<redacted>";
pub const PASSWORD_LENGTH: usize = 32;

pub const DEFAULT_OPERATOR_NAMESPACE: &str = "stackable-operators";
// TODO (Techassi): Change this to "stackable" once we switch to this version.
// Currently lots of demos can only run in the default namespace, so we have to
// keep "default" here, until we switch the demos. We can't switch them right
// now, as the old stackablectl would break.
pub const DEFAULT_NAMESPACE: &str = "default";

pub const DEFAULT_LOCAL_CLUSTER_NAME: &str = "stackable-data-platform";

pub const DEFAULT_AUTO_PURGE_INTERVAL: Duration = Duration::from_secs(60 * 15); // 15 minutes
pub const DEFAULT_CACHE_MAX_AGE: Duration = Duration::from_secs(60 * 60); // One hour
pub const CACHE_LAST_AUTO_PURGE_FILEPATH: &str = ".cache-last-purge";
pub const CACHE_PROTECTED_FILES: &[&str] = &[".cache-last-purge"];

pub const HELM_REPO_NAME_STABLE: &str = "stackable-stable";
pub const HELM_REPO_NAME_TEST: &str = "stackable-test";
pub const HELM_REPO_NAME_DEV: &str = "stackable-dev";
pub const HELM_REPO_INDEX_FILE: &str = "index.yaml";

pub const HELM_OCI_BASE: &str = "oci.stackable.tech";
pub const HELM_OCI_REGISTRY: &str = "oci://oci.stackable.tech/sdp-charts";

pub const HELM_DEFAULT_CHART_VERSION: &str = "0.0.0-dev";

/// Tuple of (product name, group, version, kind)
/// Group is usually `<product name>.stackable.tech`.
/// The version is currently hard-coded to `v1alpha1`.
/// Kind is usually `<product name with a capitalized first letter>Cluster`.
/// But there are exceptions.
pub const PRODUCTS: &[(&str, &str, &str, &str)] = &[
    (
        "airflow",
        "airflow.stackable.tech",
        "v1alpha1",
        "AirflowCluster",
    ),
    ("druid", "druid.stackable.tech", "v1alpha1", "DruidCluster"),
    ("hbase", "hbase.stackable.tech", "v1alpha1", "HbaseCluster"),
    ("hdfs", "hdfs.stackable.tech", "v1alpha1", "HdfsCluster"),
    ("hive", "hive.stackable.tech", "v1alpha1", "HiveCluster"),
    ("kafka", "kafka.stackable.tech", "v1alpha1", "KafkaCluster"),
    ("nifi", "nifi.stackable.tech", "v1alpha1", "NifiCluster"),
    ("opa", "opa.stackable.tech", "v1alpha1", "OpaCluster"),
    // Kind is `OpenSearchCluster` instead of `OpensearchCluster`.
    (
        "opensearch",
        "opensearch.stackable.tech",
        "v1alpha1",
        "OpenSearchCluster",
    ),
    // Group is `spark.stackable.tech` instead of `spark-connect.stackable.tech`.
    // Kind is `SparkConnectServer` instead of `Spark-connectCluster`.
    (
        "spark-connect",
        "spark.stackable.tech",
        "v1alpha1",
        "SparkConnectServer",
    ),
    // Group is `spark.stackable.tech` instead of `spark-history.stackable.tech`.
    // Kind is `SparkHistoryServer` instead of `Spark-historyCluster`.
    (
        "spark-history",
        "spark.stackable.tech",
        "v1alpha1",
        "SparkHistoryServer",
    ),
    (
        "superset",
        "superset.stackable.tech",
        "v1alpha1",
        "SupersetCluster",
    ),
    ("trino", "trino.stackable.tech", "v1alpha1", "TrinoCluster"),
    (
        "zookeeper",
        "zookeeper.stackable.tech",
        "v1alpha1",
        "ZookeeperCluster",
    ),
];

pub const OCI_INDEX_PAGE_SIZE: usize = 20;
