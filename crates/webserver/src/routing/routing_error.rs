use crate::http::response::Response;
use crate::http::status_code::StatusCode;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

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
    pub fn new(status: StatusCode, message: impl Into<String>) -> Self {
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
