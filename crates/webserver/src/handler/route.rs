use crate::configuration::action::Action;
use crate::configuration::configuration::Configuration;
use crate::configuration::route::Route;
use crate::handler::handler_error::HandlerError;
use crate::handler::{fixed, proxy, redirect};
use crate::http::request::Request;
use crate::http::response::Response;

pub fn handle(request: &Request, route: &Route, configuration: &Configuration) -> Result<Response, HandlerError> {
    match &route.action {
        Action::Fixed { root, fallback } => fixed::handle(root, fallback, request, configuration),
        Action::Proxy { upstream } => proxy::handle(upstream, request, configuration),
        Action::Redirect { to, code } => redirect::handle(to, code, request, configuration)
    }
}