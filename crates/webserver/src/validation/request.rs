use crate::http::method::Method;
use crate::http::request::Request;
use crate::http::status_code::StatusCode;
use crate::validation::validation_error::ValidationError;

pub fn validate(request: &Request) -> Result<(), ValidationError> {

    if request.request_line.method == Method::Trace {
        return Err(ValidationError::new(StatusCode::MethodNotAllowed, "Trace not allowed"));
    }

    if request.headers.host().is_none() {
        return Err(ValidationError::new(StatusCode::BadRequest, "Host missing"));
    }

    Ok(())
}