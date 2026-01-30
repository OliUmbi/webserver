use crate::configuration::configuration::Configuration;
use crate::configuration::path::Path;
use crate::configuration::route::Route;
use crate::http::request::Request;
use crate::http::status_code::StatusCode;
use crate::routing::{exact, prefix, regex};
use crate::routing::routing_error::RoutingError;

pub fn resolve<'a>(request: &Request, configuration: &'a Configuration) -> Result<&'a Route, RoutingError> {

    for route in &configuration.routes {
        if match &route.path {
            Path::Exact(path) => exact::matches(path, request),
            Path::Prefix(path) => prefix::matches(path, request),
            Path::Regex(path) => regex::matches(path, request)
        } {
            return Ok(route)
        }
    }
    
    Err(RoutingError::new(StatusCode::NotFound, "No matching route found"))
}