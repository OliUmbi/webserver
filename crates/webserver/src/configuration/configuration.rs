use crate::configuration::validate::Validate;
use crate::configuration::configuration_error::ConfigurationError;
use crate::configuration::route::Route;
use crate::configuration::server::Server;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Configuration {
    #[serde(default)]
    pub server: Server,

    #[serde(default)]
    pub routes: Vec<Route>,
}

impl Validate for Configuration {
    fn valid(&self) -> Result<(), ConfigurationError> {
        self.server.valid()?;
        self.routes.iter().try_for_each(Validate::valid)
    }
}
