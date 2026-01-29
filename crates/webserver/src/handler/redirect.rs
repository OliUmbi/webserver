use crate::configuration::configuration::Configuration;
use crate::configuration::route::Route;
use crate::handler::handler_error::HandlerError;
use crate::http::request::Request;
use crate::http::response::Response;

pub fn handle(request: &Request, route: &Route, configuration: &Configuration) -> Result<Response, HandlerError> {
    todo!()
}