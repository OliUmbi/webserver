use std::fs;
use crate::configuration::configuration::Configuration;
use crate::configuration::route::Route;
use crate::handler::handler_error::HandlerError;
use crate::http::headers::Headers;
use crate::http::request::Request;
use crate::http::response::Response;
use crate::http::status_code::StatusCode;

pub fn handle(request: &Request, route: &Route, configuration: &Configuration) -> Result<Response, HandlerError> {
    
    let mut path = match request_line.url.raw.as_str() {
        "/" => "/index.html",
        value => value,
    };

    path = match fs::exists(format!(
        "C:/Users/olive/IdeaProjects/webserver/examples/demo/{}",
        path
    )) {
        Ok(exists) => {
            if exists {
                path
            } else {
                "/notfound.html"
            }
        }
        Err(_) => "/notfound.html",
    };

    let body = fs::read_to_string(format!(
        "C:/Users/olive/IdeaProjects/webserver/examples/demo/{}",
        path
    ))
        .unwrap();

    let mut response_headers = Headers::new();
    response_headers.add(
        "Content-Type".to_string(),
        if path.contains("html") {
            "text/html".to_string()
        } else {
            "text/css".to_string()
        },
    );
    response_headers.add("Content-Length".to_string(), body.len().to_string());

    Response::new(StatusCode::Ok, response_headers, body.to_string())

}