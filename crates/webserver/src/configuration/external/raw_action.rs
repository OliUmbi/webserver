use crate::configuration::configuration_error::ConfigurationError;
use crate::configuration::internal::action::Action;
use crate::http::status_code::StatusCode;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub enum RawAction {
    Fixed {
        root: String,
        fallback: Option<String>,
    },
    Proxy {
        location: String,
    },
    Redirect {
        location: String,
        status_code: usize,
    },
}

impl TryFrom<RawAction> for Action {
    type Error = ConfigurationError;

    fn try_from(raw: RawAction) -> Result<Self, Self::Error> {
        match raw {
            RawAction::Fixed { root, fallback } => {
                let root = PathBuf::from(root);
                let fallback = fallback.map(|fallback| PathBuf::from(fallback));

                Ok(Action::Fixed { root, fallback })
            }
            RawAction::Proxy { location } => Ok(Action::Proxy { location }),
            RawAction::Redirect {
                location,
                status_code,
            } => {
                let status_code = StatusCode::try_from(status_code as u16).map_err(|_| {
                    ConfigurationError::new(format!("Invalid status code: {}", status_code))
                })?;

                Ok(Action::Redirect {
                    location,
                    status_code,
                })
            }
        }
    }
}
