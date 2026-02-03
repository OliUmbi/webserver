use crate::configuration::configuration_error::ConfigurationError;
use crate::configuration::internal::path::Path;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum RawPath {
    Exact(String),
    Prefix(String),
    Regex(String),
}

impl TryFrom<RawPath> for Path {
    type Error = ConfigurationError;

    fn try_from(raw: RawPath) -> Result<Self, Self::Error> {
        match raw {
            RawPath::Exact(raw) => Ok(Path::Exact(raw)),
            RawPath::Prefix(raw) => Ok(Path::Prefix(raw)),
            RawPath::Regex(raw) => {
                let regex = Regex::new(raw.as_str()).map_err(|error| {
                    ConfigurationError::new(format!("Failed to parse regex: {}", error))
                })?;

                Ok(Path::Regex(regex))
            }
        }
    }
}
