use crate::configuration::configuration::Configuration;
use crate::handler::handler_error::HandlerError;
use crate::http::headers::Headers;
use crate::http::request::Request;
use crate::http::response::Response;
use crate::http::status_code::StatusCode;

pub fn handle(to: &String, code: &StatusCode, request: &Request, configuration: &Configuration) -> Result<Response, HandlerError> {

    let mut headers = Headers::new();
    headers.add("Location", to);

    Ok(Response::new(*code, headers, Vec::new()))
}