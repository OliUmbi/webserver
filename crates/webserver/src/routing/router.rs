use crate::configuration::configuration::Configuration;
use crate::configuration::path::Path;
use crate::configuration::route::Route;
use crate::http::request::Request;
use crate::http::status_code::StatusCode;
use crate::routing::exact;
use crate::routing::routing_error::RoutingError;

pub fn resolve(request: &Request, configuration: &Configuration) -> Result<Route, RoutingError> {

    for route in configuration.routes {
        if match route.path {
            Path::Exact(_) => exact::matches(route, request),
            Path::Prefix(_) => {}
            Path::Regex(_) => {}
        } {
            return Ok(route)
        }
    }
    
    Err(RoutingError::new(StatusCode::NotFound, "No matching route found"))
}