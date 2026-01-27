use std::fs;
use crate::configuration::configuration::Configuration;
use crate::configuration::configuration_error::ConfigurationError;
use crate::configuration::validate::Validate;

pub fn parse_configuration(path: &str) -> Result<Configuration, ConfigurationError> {
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => return Err(ConfigurationError::new(format!("File not found: {}", path)))
    };

    let configuration = match toml::from_str::<Configuration>(&content) {
        Ok(configuration) => configuration,
        Err(error) => return Err(ConfigurationError::new(format!("Parse error: {}", error.message())))
    };

    configuration.valid().map(|_| configuration)
}