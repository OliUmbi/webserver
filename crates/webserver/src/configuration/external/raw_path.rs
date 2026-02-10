use crate::configuration::configuration_error::ConfigurationError;
use crate::configuration::internal::path::Path;
use crate::http::method::Method;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum RawPath {
    Exact {
        exact: String,
        methods: Option<String>,
    },
    Prefix {
        prefix: String,
        methods: Option<String>,
    },
    Regex {
        regex: String,
        methods: Option<String>,
    },
}

impl TryFrom<RawPath> for Path {
    type Error = ConfigurationError;

    fn try_from(raw: RawPath) -> Result<Self, Self::Error> {
        match raw {
            RawPath::Exact { exact, methods } => Ok(Path::Exact {
                exact,
                methods: parse_methods(methods)?,
            }),
            RawPath::Prefix { prefix, methods } => Ok(Path::Prefix {
                prefix,
                methods: parse_methods(methods)?,
            }),
            RawPath::Regex { regex, methods } => {
                let regex = Regex::new(regex.as_str()).map_err(|error| {
                    ConfigurationError::new(format!("Failed to parse regex: {}", error))
                })?;

                Ok(Path::Regex {
                    regex,
                    methods: parse_methods(methods)?,
                })
            }
        }
    }
}

fn parse_methods(methods: Option<String>) -> Result<Vec<Method>, ConfigurationError> {
    match methods {
        None => Ok(vec![
            Method::Get,
            Method::Head,
            Method::Post,
            Method::Put,
            Method::Delete,
            Method::Connect,
            Method::Options,
        ]),
        Some(methods) => methods
            .split(",")
            .map(|method| {
                Method::from_str(method).map_err(|_| {
                    ConfigurationError::new(format!("Failed to parse method: {}", method))
                })
            })
            .collect(),
    }
}
