use crate::http::response::Response;
use crate::http::status_code::StatusCode;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct HandlerError {
    pub status: StatusCode,
    pub message: String,
}

impl Display for HandlerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.status.as_str(), self.message)
    }
}

impl Error for HandlerError {}

impl HandlerError {
    pub fn new(status: StatusCode, message: impl Into<String>) -> Self {
        Self {
            status,
            message: message.into(),
        }
    }
}

impl From<HandlerError> for Response {
    fn from(error: HandlerError) -> Self {
        Response::error(error.status, error.message)
    }
}
