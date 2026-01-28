use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::configuration::configuration_error::ConfigurationError;
use crate::configuration::validate::Validate;
use crate::http::status_code::StatusCode;

#[derive(Debug, Deserialize, Serialize)]
pub enum Action {
    Fixed { root: PathBuf, fallback: Option<PathBuf> },
    Proxy { upstream: String },
    Redirect { to: String, code: StatusCode },
}

impl Validate for Action {
    fn valid(&self) -> Result<(), ConfigurationError> {
        Ok(()) // todo implement
    }
}
