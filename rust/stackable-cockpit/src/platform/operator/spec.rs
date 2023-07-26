use std::{fmt::Display, str::FromStr};

use snafu::Snafu;
use tracing::{info, instrument};

use crate::{
    constants::{HELM_REPO_NAME_DEV, HELM_REPO_NAME_STABLE, HELM_REPO_NAME_TEST},
    helm,
    utils::operator_chart_name,
};

pub const VALID_OPERATORS: &[&str] = &[
    "airflow",
    "commons",
    "druid",
    "hbase",
    "hdfs",
    "hive",
    "kafka",
    "listener",
    "nifi",
    "opa",
    "secret",
    "spark-k8s",
    "superset",
    "trino",
    "zookeeper",
];

#[derive(Debug, Snafu, PartialEq)]
pub enum OperatorSpecParseError {
    #[snafu(display("invalid equal sign count in operator spec, expected one"))]
    InvalidEqualSignCount,

    #[snafu(display("invalid spec version"))]
    InvalidSpecVersion,

    #[snafu(display("invalid (empty) operator spec input"))]
    InvalidSpecInput,

    #[snafu(display("invalid operator name: '{name}'"))]
    InvalidName { name: String },
}

/// OperatorSpec describes the format of an operator name with optional version number. The string format is
/// `<OPERATOR_NAME>(=<VERSION>)`. Valid values values are: `operator`, `operator=1.2.3` or `operator=1.2.3-rc1`.
#[derive(Clone, Debug)]
pub struct OperatorSpec {
    pub version: Option<String>,
    pub name: String,
}

impl Display for OperatorSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.name,
            match &self.version {
                Some(v) => format!("={v}"),
                None => "".into(),
            }
        )
    }
}

impl FromStr for OperatorSpec {
    type Err = OperatorSpecParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = s.trim();

        // Empty input is not allowed
        if input.is_empty() {
            return Err(OperatorSpecParseError::InvalidSpecInput);
        }

        // Split at each equal sign
        let parts: Vec<&str> = input.split('=').collect();
        let len = parts.len();

        // If there are more than 2 equal signs, return error
        // because of invalid spec format
        if len > 2 {
            return Err(OperatorSpecParseError::InvalidEqualSignCount);
        }

        // If there is only one part, the input didn't include
        // the optional version identifier
        if len == 1 {
            return Ok(Self {
                name: input.into(),
                version: None,
            });
        }

        // If there is an equal sign, but no version after
        if parts[1].is_empty() {
            return Err(OperatorSpecParseError::InvalidSpecVersion);
        }

        if !VALID_OPERATORS.contains(&parts[0]) {
            return Err(OperatorSpecParseError::InvalidName {
                name: parts[0].to_string(),
            });
        }

        // There are two parts, so an operator name and version
        Ok(Self {
            name: parts[0].into(),
            version: Some(parts[1].into()),
        })
    }
}

impl TryFrom<String> for OperatorSpec {
    type Error = OperatorSpecParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(&value)
    }
}

impl TryFrom<&str> for OperatorSpec {
    type Error = OperatorSpecParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_string())
    }
}

impl OperatorSpec {
    pub fn new<T>(name: T, version: Option<String>) -> Result<Self, OperatorSpecParseError>
    where
        T: AsRef<str>,
    {
        let name = name.as_ref();

        if !VALID_OPERATORS.contains(&name) {
            return Err(OperatorSpecParseError::InvalidName {
                name: name.to_string(),
            });
        }

        Ok(Self {
            version,
            name: name.to_string(),
        })
    }

    /// Returns the name used by Helm
    pub fn helm_name(&self) -> String {
        operator_chart_name(&self.name)
    }

    /// Returns the repo used by Helm based on the specified version
    pub fn helm_repo_name(&self) -> String {
        match &self.version {
            Some(version) if version.ends_with("-nightly") => HELM_REPO_NAME_DEV,
            Some(version) if version.ends_with("-dev") => HELM_REPO_NAME_DEV,
            Some(version) if version.contains("-pr") => HELM_REPO_NAME_TEST,
            Some(_) => HELM_REPO_NAME_STABLE,
            None => HELM_REPO_NAME_DEV,
        }
        .into()
    }

    /// Installs the operator using Helm.
    #[instrument(skip_all)]
    pub fn install(&self, namespace: &str) -> Result<(), helm::HelmError> {
        info!("Installing operator {}", self);

        let helm_name = self.helm_name();
        let helm_repo = self.helm_repo_name();
        let version = self.version.as_deref();

        // Install using Helm
        match helm::install_release_from_repo(
            &self.name,
            &helm_name,
            helm::ChartVersion {
                repo_name: &helm_repo,
                chart_name: &helm_name,
                chart_version: version,
            },
            None,
            namespace,
            true,
        ) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Uninstalls the operator using Helm.
    #[instrument]
    pub fn uninstall<T>(&self, namespace: T) -> Result<(), helm::HelmError>
    where
        T: AsRef<str> + std::fmt::Debug,
    {
        match helm::uninstall_release(&self.helm_name(), namespace.as_ref(), true) {
            Ok(status) => {
                println!("{status}");
                Ok(())
            }
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::platform::operator::{OperatorSpec, OperatorSpecParseError};

    #[test]
    fn simple_operator_spec() {
        match OperatorSpec::try_from("operator") {
            Ok(spec) => {
                assert_eq!(spec.name, String::from("operator"));
                assert_eq!(spec.version, None);
            }
            Err(err) => panic!("{err}"),
        }
    }

    #[test]
    fn version_operator_spec() {
        match OperatorSpec::try_from("zookeeper=1.2.3") {
            Ok(spec) => {
                assert_eq!(spec.name, String::from("zookeeper"));
                assert_eq!(spec.version, Some(String::from("1.2.3")));
            }
            Err(err) => panic!("{err}"),
        }
    }

    #[test]
    fn empty_operator_spec() {
        match OperatorSpec::try_from("") {
            Ok(spec) => panic!("SHOULD FAIL: {spec}"),
            Err(err) => assert_eq!(err, OperatorSpecParseError::InvalidSpecInput),
        }
    }

    #[test]
    fn empty_version_operator_spec() {
        match OperatorSpec::try_from("operator=") {
            Ok(spec) => panic!("SHOULD FAIL: {spec}"),
            Err(err) => assert_eq!(err, OperatorSpecParseError::InvalidSpecVersion),
        }
    }

    #[test]
    fn invalid_version_operator_spec() {
        match OperatorSpec::try_from("operator=1.2.3=") {
            Ok(spec) => panic!("SHOULD FAIL: {spec}"),
            Err(err) => assert_eq!(err, OperatorSpecParseError::InvalidEqualSignCount),
        }
    }
}
