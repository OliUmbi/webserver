use crate::configuration::internal::action::Action;
use crate::configuration::internal::configuration::Configuration;
use crate::configuration::internal::route::Route;
use crate::handler::handler_error::HandlerError;
use crate::handler::{fixed, proxy, redirect};
use crate::http::request::Request;
use crate::http::response::Response;
use crate::server::connection::Connection;

pub fn handle(
    request: &mut Request,
    route: &Route,
    connection: &mut Connection,
    configuration: &Configuration,
) -> Result<Response, HandlerError> {
    match &route.action {
        Action::Fixed { root, fallback } => fixed::handle(root, fallback, request),
        Action::Proxy { location } => proxy::handle(location, request, connection, configuration),
        Action::Redirect {
            location,
            status_code,
        } => redirect::handle(location, status_code),
    }
}
