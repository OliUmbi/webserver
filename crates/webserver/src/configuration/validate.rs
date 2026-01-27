use crate::configuration::configuration_error::ConfigurationError;

pub trait Validate {
    fn valid(&self) -> Result<(), ConfigurationError>;
}