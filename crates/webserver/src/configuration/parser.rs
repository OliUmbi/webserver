use crate::configuration::configuration_error::ConfigurationError;
use crate::configuration::external::raw_configuration::RawConfiguration;
use crate::configuration::internal::configuration::Configuration;
use std::fs;

pub fn parse_configuration(path: String) -> Result<Configuration, ConfigurationError> {
    let content = match fs::read_to_string(&path) {
        Ok(content) => content,
        Err(_) => return Err(ConfigurationError::new(format!("File not found: {}", path))),
    };

    let raw_configuration = match toml::from_str::<RawConfiguration>(&content) {
        Ok(configuration) => configuration,
        Err(error) => {
            return Err(ConfigurationError::new(format!(
                "Failed to parse configuration file: {}",
                error.message()
            )));
        }
    };

    Configuration::try_from(raw_configuration)
}
