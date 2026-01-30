use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct ConfigurationError {
    pub message: String,
}

impl Display for ConfigurationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ConfigurationError {}

impl ConfigurationError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into()
        }
    }
}



