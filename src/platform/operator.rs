use std::{fmt::Display, str::FromStr};

use thiserror::Error;

#[derive(Clone, Debug)]
pub struct OperatorSpec {
    pub operator_name: String,
    pub version: Option<String>,
}

#[derive(Debug, Error, PartialEq)]
pub enum OperatorSpecParseError {
    #[error("invalid equal sign count in operator spec, expected one")]
    InvalidEqualSignCount,

    #[error("invalid spec version")]
    InvalidSpecVersion,

    #[error("invalid (empty) operator spec input")]
    InvalidSpecInput,
}

impl Display for OperatorSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.operator_name,
            match &self.version {
                Some(v) => format!("={v}"),
                None => "".into(),
            }
        )
    }
}

impl TryFrom<String> for OperatorSpec {
    type Error = OperatorSpecParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let input = value.trim();

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
                operator_name: input.into(),
                version: None,
            });
        }

        // If there is an equal sign, but no version after
        if parts[1].is_empty() {
            return Err(OperatorSpecParseError::InvalidSpecVersion);
        }

        // There are two parts, so an operator name and version
        Ok(Self {
            operator_name: parts[0].into(),
            version: Some(parts[1].into()),
        })
    }
}

impl FromStr for OperatorSpec {
    type Err = OperatorSpecParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s.to_string())
    }
}

impl TryFrom<&str> for OperatorSpec {
    type Error = OperatorSpecParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_string())
    }
}

#[cfg(test)]
mod test {
    use crate::platform::operator::{OperatorSpec, OperatorSpecParseError};

    #[test]
    fn test_simple_operator_spec() {
        match OperatorSpec::try_from("operator") {
            Ok(spec) => {
                assert_eq!(spec.operator_name, String::from("operator"));
                assert_eq!(spec.version, None);
            }
            Err(err) => panic!("{err}"),
        }
    }

    #[test]
    fn test_version_operator_spec() {
        match OperatorSpec::try_from("operator=1.2.3") {
            Ok(spec) => {
                assert_eq!(spec.operator_name, String::from("operator"));
                assert_eq!(spec.version, Some(String::from("1.2.3")));
            }
            Err(err) => panic!("{err}"),
        }
    }

    #[test]
    fn test_empty_operator_spec() {
        match OperatorSpec::try_from("") {
            Ok(spec) => panic!("SHOULD FAIL: {spec}"),
            Err(err) => assert_eq!(err, OperatorSpecParseError::InvalidSpecInput),
        }
    }

    #[test]
    fn test_empty_version_operator_spec() {
        match OperatorSpec::try_from("operator=") {
            Ok(spec) => panic!("SHOULD FAIL: {spec}"),
            Err(err) => assert_eq!(err, OperatorSpecParseError::InvalidSpecVersion),
        }
    }

    #[test]
    fn test_invalid_version_operator_spec() {
        match OperatorSpec::try_from("operator=1.2.3=") {
            Ok(spec) => panic!("SHOULD FAIL: {spec}"),
            Err(err) => assert_eq!(err, OperatorSpecParseError::InvalidEqualSignCount),
        }
    }
}
