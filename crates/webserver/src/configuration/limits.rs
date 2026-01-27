use serde::{Deserialize, Serialize};
use crate::configuration::configuration_error::ConfigurationError;
use crate::configuration::validate::Validate;

#[derive(Debug, Deserialize, Serialize)]
pub struct Limits {

    #[serde(default = "default_max_header_length")]
    pub max_header_length: usize,

    #[serde(default = "default_max_body_length")]
    pub max_body_length: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            max_header_length: default_max_header_length(),
            max_body_length: default_max_body_length(),
        }
    }
}

impl Validate for Limits {
    fn valid(&self) -> Result<(), ConfigurationError> {

        if self.max_header_length == 0 {
            return Err(ConfigurationError::new("Max header length must be more than 0"))
        }

        if self.max_body_length == 0 {
            return Err(ConfigurationError::new("Max body length must be more than 0"))
        }
        
        Ok(())
    }
}

fn default_max_header_length() -> usize { 8 * 1024 }
fn default_max_body_length() -> usize { 1024 * 1024 }
