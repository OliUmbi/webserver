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
            Path::Exact { exact, methods } => exact::matches(exact, methods, request),
            Path::Prefix { prefix, methods } => prefix::matches(prefix, methods, request),
            Path::Regex { regex, methods } => regex::matches(regex, methods, request),
        } {
            return Ok(route);
        }
    }

    Err(RoutingError::new(
        StatusCode::NotFound,
        "No matching route found",
    ))
}
