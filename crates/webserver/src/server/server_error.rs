use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

// todo rethink splitting request errors 
#[derive(Debug)]
pub struct ServerError {
    pub message: String,
}

impl Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ServerError {}

impl ServerError {
    pub fn new<S: Into<String>>(message: S) -> Self {
        Self {
            message: message.into()
        }
    }
}



