use std::str::FromStr;

use thiserror::Error;

pub struct RawStackParameter {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Error)]
pub enum RawStackParameterParseError {
    #[error("invalid equal sign count in stack parameter, expected one")]
    InvalidEqualSignCount,

    #[error("invalid stack parameter value, cannot be empty")]
    InvalidParameterValue,

    #[error("invalid (empty) stack parameter input")]
    InvalidParameterInput,
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
    use crate::types::platform::RawStackParameter;

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
}
