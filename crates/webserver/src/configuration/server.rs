use std::time::Duration;
use serde::{Deserialize, Serialize};
use crate::configuration::configuration_error::ConfigurationError;
use crate::configuration::limits::Limits;
use crate::configuration::validate::Validate;

#[derive(Debug, Deserialize, Serialize)]
pub struct Server {
    #[serde(default = "default_threads")]
    pub threads: usize,

    #[serde(default = "default_connections")]
    pub connections: usize,

    #[serde(default = "default_port")]
    pub port: u16,

    #[serde(default = "default_timeout")]
    pub timeout: Duration,

    #[serde(default)]
    pub limits: Limits,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            threads: default_threads(),
            connections: default_connections(),
            port: default_port(),
            timeout: default_timeout(),
            limits: Limits::default()
        }
    }
}

impl Validate for Server {
    fn valid(&self) -> Result<(), ConfigurationError> {
        if self.threads == 0 {
            return Err(ConfigurationError::new("Threads must be more than 0"))
        }

        if self.connections == 0 {
            return Err(ConfigurationError::new("Connections must be more than 0"))
        }

        if self.port == 0 {
            return Err(ConfigurationError::new("Port cannot be 0"))
        }

        if self.timeout.is_zero() {
            return Err(ConfigurationError::new("Timeout must be more than 0"))
        }

        self.limits.valid()?;

        Ok(())
    }
}

fn default_threads() -> usize { 4 }
fn default_connections() -> usize { 1024 }
fn default_port() -> u16 { 80 }
fn default_timeout() -> Duration { Duration::from_secs(10) }
