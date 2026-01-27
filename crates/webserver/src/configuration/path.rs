use serde::{Deserialize, Serialize};
use crate::configuration::configuration_error::ConfigurationError;
use crate::configuration::validate::Validate;

#[derive(Debug, Deserialize, Serialize)]
pub enum Path {
    Exact(String),
    Prefix(String),
    Regex(String),
}

impl Validate for Path {
    fn valid(&self) -> Result<(), ConfigurationError> {
        Ok(()) // todo implement
    }
}
