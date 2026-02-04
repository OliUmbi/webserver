use crate::configuration::configuration_error::ConfigurationError;
use crate::configuration::external::raw_route::RawRoute;
use crate::configuration::external::raw_server::RawServer;
use crate::configuration::internal::configuration::Configuration;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RawConfiguration {
    pub headless: Option<bool>,

    #[serde(default)]
    pub server: RawServer,

    #[serde(default)]
    pub routes: Vec<RawRoute>,
}

impl TryFrom<RawConfiguration> for Configuration {
    type Error = ConfigurationError;

    fn try_from(raw: RawConfiguration) -> Result<Self, Self::Error> {
        Ok(Self {
            headless: raw.headless.unwrap_or(true),
            server: raw.server.try_into()?,
            routes: raw
                .routes
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
        })
    }
}
