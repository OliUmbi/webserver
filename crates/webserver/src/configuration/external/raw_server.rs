use std::time::Duration;
use serde::{Deserialize, Serialize};
use crate::configuration::configuration_error::ConfigurationError;
use crate::configuration::internal::server::Server;

#[derive(Debug, Deserialize, Serialize)]
pub struct RawServer {
    #[serde(default = "default_threads")]
    pub threads: usize,

    #[serde(default = "default_connections")]
    pub connections: usize,

    #[serde(default = "default_port")]
    pub port: usize,

    #[serde(default = "default_timeout_in_secs")]
    pub timeout_in_secs: usize,

    #[serde(default = "default_max_header_length")]
    pub max_header_length: usize,

    #[serde(default = "default_max_body_length")]
    pub max_body_length: usize,
}

impl Default for RawServer {
    fn default() -> Self {
        Self {
            threads: default_threads(),
            connections: default_connections(),
            port: default_port(),
            timeout_in_secs: default_timeout_in_secs(),
            max_header_length: default_max_header_length(),
            max_body_length: default_max_body_length(),
        }
    }
}

fn default_threads() -> usize { 4 }
fn default_connections() -> usize { 1024 }
fn default_port() -> usize { 80 }
fn default_timeout_in_secs() -> usize { 10 }
fn default_max_header_length() -> usize { 8 * 1024 }
fn default_max_body_length() -> usize { 1024 * 1024 }

impl TryFrom<RawServer> for Server {
    type Error = ConfigurationError;

    fn try_from(raw: RawServer) -> Result<Self, Self::Error> {
        if raw.threads == 0 {
            return Err(ConfigurationError::new("Threads must be more than 0"))
        }

        if raw.connections == 0 {
            return Err(ConfigurationError::new("Connections must be more than 0"))
        }

        if raw.port == 0 && raw.port > 65535 {
            return Err(ConfigurationError::new("Port cannot be 0"))
        }

        if raw.timeout_in_secs == 0 {
            return Err(ConfigurationError::new("Timeout must be more than 0"))
        }

        if raw.max_header_length == 0 {
            return Err(ConfigurationError::new("Max header length must be more than 0"))
        }

        if raw.max_body_length == 0 {
            return Err(ConfigurationError::new("Max body length must be more than 0"))
        }
        
        Ok(Server {
            threads: raw.threads,
            connections: raw.connections,
            port: raw.port,
            timeout: Duration::from_secs(raw.timeout_in_secs as u64),
            max_header_length: raw.max_header_length,
            max_body_length: raw.max_body_length
        })
    }
}
