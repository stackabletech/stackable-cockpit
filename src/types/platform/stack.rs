use std::{
    fmt::{Display, Formatter, Write},
    str::FromStr,
};

use thiserror::Error;

#[derive(Debug, PartialEq)]
pub struct RawStackParameter {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Error, PartialEq)]
pub enum RawStackParameterParseError {
    #[error("invalid equal sign count in stack parameter, expected one")]
    InvalidEqualSignCount,

    #[error("invalid stack parameter value, cannot be empty")]
    InvalidParameterValue,

    #[error("invalid stack parameter name, cannot be empty")]
    InvalidParameterName,

    #[error("invalid (empty) stack parameter input")]
    InvalidParameterInput,
}

impl Display for RawStackParameter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}", self.name, self.value)
    }
}

impl FromStr for RawStackParameter {
    type Err = RawStackParameterParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = s.trim();

        // Empty input is not allowed
        if input.is_empty() {
            return Err(RawStackParameterParseError::InvalidParameterInput);
        }

        // Split at each equal sign
        let parts: Vec<&str> = input.split('=').collect();
        let len = parts.len();

        // If there are more than 2 equal signs, return error
        // because of invalid spec format
        if len > 2 {
            return Err(RawStackParameterParseError::InvalidEqualSignCount);
        }

        // Only specifying a key is not valid
        if len == 1 {
            return Err(RawStackParameterParseError::InvalidParameterValue);
        }

        // If there is an equal sign, but no key before
        if parts[0].is_empty() {
            return Err(RawStackParameterParseError::InvalidParameterName);
        }

        // If there is an equal sign, but no value after
        if parts[1].is_empty() {
            return Err(RawStackParameterParseError::InvalidParameterValue);
        }

        Ok(Self {
            name: parts[0].to_string(),
            value: parts[1].to_string(),
        })
    }
}

impl TryFrom<String> for RawStackParameter {
    type Error = RawStackParameterParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(value.as_str())
    }
}

impl TryFrom<&str> for RawStackParameter {
    type Error = RawStackParameterParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

pub struct RawStackParameters(Vec<RawStackParameter>);

impl Display for RawStackParameters {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // We sadly cannot use .join() here, thats why we need to do it the manual way...
        let mut s = String::new();
        for param in &self.0 {
            write!(&mut s, "{} ", param)?;
        }
        write!(f, "{}", s.trim_end())
    }
}

impl FromStr for RawStackParameters {
    type Err = RawStackParameterParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = s.trim();

        if input.is_empty() {
            return Err(RawStackParameterParseError::InvalidParameterInput);
        }

        let mut params = Vec::new();

        let parts: Vec<&str> = input.split(" ").collect();
        for part in parts {
            let param: RawStackParameter = part.parse()?;
            params.push(param);
        }

        Ok(Self(params))
    }
}

impl TryFrom<String> for RawStackParameters {
    type Error = RawStackParameterParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(value.as_str())
    }
}

impl TryFrom<&str> for RawStackParameters {
    type Error = RawStackParameterParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl RawStackParameters {
    /// Returns an iterator over the raw stack parameters
    pub fn iter(&self) -> RawStackParameterIter<'_> {
        RawStackParameterIter {
            params: self,
            index: 0,
        }
    }

    /// Returns the number of parameters
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns if there are no parameters
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

pub struct RawStackParameterIter<'a> {
    params: &'a RawStackParameters,
    index: usize,
}

impl<'a> Iterator for RawStackParameterIter<'a> {
    type Item = &'a RawStackParameter;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.params.0.len() {
            return None;
        }

        let current = self.params.0.get(self.index);
        self.index += 1;

        current
    }
}

#[cfg(test)]
mod test {
    use crate::types::platform::{
        RawStackParameter, RawStackParameterParseError, RawStackParameters,
    };

    #[test]
    fn single_parameter_str() {
        match RawStackParameter::try_from("param=value") {
            Ok(param) => {
                assert_eq!(param.name, "param".to_string());
                assert_eq!(param.value, "value".to_string());
            }
            Err(err) => panic!("{err}"),
        }
    }

    #[test]
    fn single_parameter_string() {
        match RawStackParameter::try_from("param=value".to_string()) {
            Ok(param) => {
                assert_eq!(param.name, "param".to_string());
                assert_eq!(param.value, "value".to_string());
            }
            Err(err) => panic!("{err}"),
        }
    }

    #[test]
    fn single_parameter_no_value() {
        match RawStackParameter::try_from("param") {
            Ok(param) => panic!("SHOULD FAIL: {param}"),
            Err(err) => assert_eq!(err, RawStackParameterParseError::InvalidParameterValue),
        }
    }

    #[test]
    fn single_parameter_equal_sign_no_value() {
        match RawStackParameter::try_from("param=") {
            Ok(param) => panic!("SHOULD FAIL: {param}"),
            Err(err) => assert_eq!(err, RawStackParameterParseError::InvalidParameterValue),
        }
    }

    #[test]
    fn single_parameter_only_equal_sign() {
        match RawStackParameter::try_from("=") {
            Ok(param) => panic!("SHOULD FAIL: {param}"),
            Err(err) => assert_eq!(err, RawStackParameterParseError::InvalidParameterName),
        }
    }

    #[test]
    fn single_parameter_multi_equal_sign() {
        match RawStackParameter::try_from("param=value=invalid") {
            Ok(param) => panic!("SHOULD FAIL: {param}"),
            Err(err) => assert_eq!(err, RawStackParameterParseError::InvalidEqualSignCount),
        }
    }

    #[test]
    fn single_parameter_multi_only_equal_sign() {
        match RawStackParameter::try_from("==") {
            Ok(param) => panic!("SHOULD FAIL: {param}"),
            Err(err) => assert_eq!(err, RawStackParameterParseError::InvalidEqualSignCount),
        }
    }

    #[test]
    fn multi_parameters_str() {
        match RawStackParameters::try_from("param1=value1 param2=value2") {
            Ok(params) => {
                assert_eq!(params.len(), 2);
                let mut iter = params.iter();

                let p = iter.next();
                assert!(p.is_some());
                assert_eq!(
                    p.unwrap(),
                    &RawStackParameter {
                        name: "param1".into(),
                        value: "value1".into()
                    }
                );

                let p = iter.next();
                assert!(p.is_some());
                assert_eq!(
                    p.unwrap(),
                    &RawStackParameter {
                        name: "param2".into(),
                        value: "value2".into()
                    }
                );

                let p = iter.next();
                assert!(p.is_none());
            }
            Err(err) => panic!("{err}"),
        }
    }

    #[test]
    fn multi_parameters_string() {
        match RawStackParameters::try_from("param1=value1 param2=value2".to_string()) {
            Ok(params) => {
                assert_eq!(params.len(), 2);
                let mut iter = params.iter();

                let p = iter.next();
                assert!(p.is_some());
                assert_eq!(
                    p.unwrap(),
                    &RawStackParameter {
                        name: "param1".into(),
                        value: "value1".into()
                    }
                );

                let p = iter.next();
                assert!(p.is_some());
                assert_eq!(
                    p.unwrap(),
                    &RawStackParameter {
                        name: "param2".into(),
                        value: "value2".into()
                    }
                );

                let p = iter.next();
                assert!(p.is_none());
            }
            Err(err) => panic!("{err}"),
        }
    }
}
