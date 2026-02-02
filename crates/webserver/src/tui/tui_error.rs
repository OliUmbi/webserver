use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use crate::http::response::Response;
use crate::http::status_code::StatusCode;
use crate::parser::parser_error::ParserError;

#[derive(Debug)]
pub struct TuiError {
    pub message: String,
}

impl Display for TuiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for TuiError {}

impl TuiError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into()
        }
    }
}
