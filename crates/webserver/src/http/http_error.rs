use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use crate::http::response::Response;
use crate::http::status_code::StatusCode;

#[derive(Debug)]
pub struct HttpError {
    pub status: StatusCode,
    pub message: String,
}

impl Display for HttpError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.status.as_str(), self.message)
    }
}

impl Error for HttpError {}

impl HttpError {
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

impl From<HttpError> for Response {
    fn from(err: HttpError) -> Self {
        Response::error(err.status, err.message)
    }
}


