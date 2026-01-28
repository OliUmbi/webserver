use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use crate::http::response::Response;
use crate::http::status_code::StatusCode;

#[derive(Debug)]
pub struct ParserError {
    pub status: StatusCode,
    pub message: String,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.status.as_str(), self.message)
    }
}

impl Error for ParserError {}

impl ParserError {
    pub fn new<S: Into<String>>(status: StatusCode, message: S) -> Self {
        Self {
            status,
            message: message.into()
        }
    }
}

impl From<ParserError> for Response {
    fn from(error: ParserError) -> Self {
        Response::error(error.status, error.message)
    }
}


