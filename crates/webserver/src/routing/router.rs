use crate::configuration::internal::configuration::Configuration;
use crate::configuration::internal::path::Path;
use crate::configuration::internal::route::Route;
use crate::http::request::Request;
use crate::http::status_code::StatusCode;
use crate::routing::routing_error::RoutingError;
use crate::routing::{exact, prefix, regex};

pub fn resolve<'a>(
    request: &Request,
    configuration: &'a Configuration,
) -> Result<&'a Route, RoutingError> {
    for route in &configuration.routes {
        if match &route.path {
            Path::Exact(path) => exact::matches(path, request),
            Path::Prefix(path) => prefix::matches(path, request),
            Path::Regex(regex) => regex::matches(regex, request),
        } {
            return Ok(route);
        }
    }

    Err(RoutingError::new(
        StatusCode::NotFound,
        "No matching route found",
    ))
}
