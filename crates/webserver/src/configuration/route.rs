use serde::{Deserialize, Serialize};
use crate::configuration::action::Action;
use crate::configuration::configuration_error::ConfigurationError;
use crate::configuration::path::Path;
use crate::configuration::validate::Validate;

// todo cache, timeout, etc
#[derive(Debug, Deserialize, Serialize)]
pub struct Route {
    pub path: Path,
    pub action: Action,
}

impl Validate for Route {
    fn valid(&self) -> Result<(), ConfigurationError> {
        self.path.valid()?;
        self.path.valid()?;

        Ok(())
    }
}
