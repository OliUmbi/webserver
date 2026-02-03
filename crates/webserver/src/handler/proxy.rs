use crate::configuration::internal::configuration::Configuration;
use crate::handler::handler_error::HandlerError;
use crate::http::request::Request;
use crate::http::response::Response;

pub fn handle(
    location: &String,
    request: &Request,
    configuration: &Configuration,
) -> Result<Response, HandlerError> {
    todo!()
}
