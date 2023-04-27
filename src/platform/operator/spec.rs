use std::{fmt::Display, str::FromStr};

use thiserror::Error;
use tracing::{info, instrument};

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

/// OperatorSpec describes the format of an operator name with optional version number. The string format is
/// `<OPERATOR_NAME>(=<VERSION>)`. Valid values values are: `operator`, `operator=1.2.3` or `operator=1.2.3-rc1`.
#[derive(Clone, Debug)]
pub struct OperatorSpec {
    pub version: Option<String>,
    pub name: String,
}

#[derive(Debug, Error, PartialEq)]
pub enum OperatorSpecParseError {
    #[error("invalid equal sign count in operator spec, expected one")]
    InvalidEqualSignCount,

    #[error("invalid spec version")]
    InvalidSpecVersion,

    #[error("invalid (empty) operator spec input")]
    InvalidSpecInput,

    #[error("invalid operator name: '{0}'")]
    InvalidName(String),
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
            return Err(OperatorSpecParseError::InvalidName(parts[0].to_string()));
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
    pub fn new(name: String, version: Option<String>) -> Result<Self, OperatorSpecParseError> {
        if !VALID_OPERATORS.contains(&name.as_str()) {
            return Err(OperatorSpecParseError::InvalidName(name));
        }

        Ok(Self { version, name })
    }

    /// Returns the name used by Helm
    pub fn helm_name(&self) -> String {
        format!("{}-operator", self.name)
    }

    /// Returns the repo used by Helm based on the specified version
    pub fn helm_repo(&self) -> String {
        match &self.version {
            Some(version) if version.ends_with("-nightly") => "stackable-dev",
            Some(version) if version.ends_with("-dev") => "stackable-dev",
            Some(version) if version.contains("-pr") => "stackable-test",
            Some(_) => "stackable-stable",
            None => "stackable-dev",
        }
        .into()
    }

    /// Installs the operator using Helm
    #[instrument(skip_all)]
    pub fn install(&self) {
        info!("Installing operator {}", self);

        let helm_name = self.helm_name();
        let helm_repo = self.helm_repo();

        // Install using Helm
        todo!()
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
        match OperatorSpec::try_from("operator=1.2.3") {
            Ok(spec) => {
                assert_eq!(spec.name, String::from("operator"));
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
