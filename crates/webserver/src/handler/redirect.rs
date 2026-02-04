use crate::handler::handler_error::HandlerError;
use crate::http::headers::Headers;
use crate::http::response::Response;
use crate::http::status_code::StatusCode;

pub fn handle(
    location: &String,
    status_code: &StatusCode
) -> Result<Response, HandlerError> {
    let mut headers = Headers::new();
    headers.add("Location", location);

    Ok(Response::new(*status_code, headers, Vec::new()))
}
