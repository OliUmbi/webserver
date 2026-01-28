use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use crate::http::response::Response;
use crate::http::status_code::StatusCode;
use crate::parser::parser_error::ParserError;

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
    pub fn new<S: Into<String>>(status: StatusCode, message: S) -> Self {
        Self {
            status,
            message: message.into()
        }
    }

    pub fn bad_request(message: String) -> Self {
        Self::new(StatusCode::BadRequest, message)
    }
}

impl From<HandlerError> for Response {
    fn from(error: HandlerError) -> Self {
        Response::error(error.status, error.message)
    }
}
