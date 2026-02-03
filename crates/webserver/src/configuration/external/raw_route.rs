use crate::configuration::configuration_error::ConfigurationError;
use crate::configuration::external::raw_action::RawAction;
use crate::configuration::external::raw_path::RawPath;
use crate::configuration::internal::route::Route;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RawRoute {
    pub path: RawPath,
    pub action: RawAction,
}

impl TryFrom<RawRoute> for Route {
    type Error = ConfigurationError;

    fn try_from(raw: RawRoute) -> Result<Self, Self::Error> {
        Ok(Self {
            path: raw.path.try_into()?,
            action: raw.action.try_into()?,
        })
    }
}
