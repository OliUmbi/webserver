use serde::{Deserialize, Serialize};
use crate::configuration::configuration_error::ConfigurationError;
use crate::configuration::external::raw_route::RawRoute;
use crate::configuration::external::raw_server::RawServer;
use crate::configuration::internal::configuration::Configuration;

#[derive(Debug, Deserialize, Serialize)]
pub struct RawConfiguration {
    #[serde(default)]
    pub server: RawServer,

    #[serde(default)]
    pub routes: Vec<RawRoute>,
}

impl TryFrom<RawConfiguration> for Configuration {
    type Error = ConfigurationError;

    fn try_from(raw: RawConfiguration) -> Result<Self, Self::Error> {
        Ok(Self {
            server: raw.server.try_into()?,
            routes: raw.routes.into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?
        })
    }
}
