use crate::configuration::configuration::Configuration;
use crate::configuration::route::Route;
use crate::http::request::Request;
use crate::routing::routing_error::RoutingError;

pub fn resolve(request: &Request, configuration: &Configuration) -> Result<Route, RoutingError> {
    todo!()
}