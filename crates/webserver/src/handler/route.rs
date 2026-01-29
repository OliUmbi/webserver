use crate::configuration::action::Action;
use crate::configuration::configuration::Configuration;
use crate::configuration::route::Route;
use crate::handler::handler_error::HandlerError;
use crate::handler::{fixed, proxy, redirect};
use crate::http::request::Request;
use crate::http::response::Response;

pub fn handle(request: &Request, route: &Route, configuration: &Configuration) -> Result<Response, HandlerError> {

    match route.action {
        Action::Fixed { .. } => fixed::handle(request, route, configuration),
        Action::Proxy { .. } => proxy::handle(request, route, configuration),
        Action::Redirect { .. } => redirect::handle(request, route, configuration)
    }
}