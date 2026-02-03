use crate::configuration::internal::action::Action;
use crate::configuration::internal::configuration::Configuration;
use crate::configuration::internal::route::Route;
use crate::handler::handler_error::HandlerError;
use crate::handler::{fixed, proxy, redirect};
use crate::http::request::Request;
use crate::http::response::Response;

pub fn handle(request: &Request, route: &Route, configuration: &Configuration) -> Result<Response, HandlerError> {
    match &route.action {
        Action::Fixed { root, fallback } => fixed::handle(root, fallback, request, configuration),
        Action::Proxy { location } => proxy::handle(location, request, configuration),
        Action::Redirect { location, status_code } => redirect::handle(location, status_code, request, configuration)
    }
}