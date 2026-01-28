use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use crate::handler::handler_error::HandlerError;
use crate::http::response::Response;
use crate::http::status_code::StatusCode;

#[derive(Debug)]
pub struct RoutingError {
    pub status: StatusCode,
    pub message: String,
}

impl Display for RoutingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.status.as_str(), self.message)
    }
}

impl Error for RoutingError {}

impl RoutingError {
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

impl From<RoutingError> for Response {
    fn from(error: RoutingError) -> Self {
        Response::error(error.status, error.message)
    }
}
