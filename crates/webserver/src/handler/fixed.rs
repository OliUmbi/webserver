use std::fs;
use std::path::{Path, PathBuf};
use crate::configuration::configuration::Configuration;
use crate::handler::handler_error::HandlerError;
use crate::http::headers::Headers;
use crate::http::media_type::{MediaType, TopLevelMediaType};
use crate::http::request::Request;
use crate::http::response::Response;
use crate::http::status_code::StatusCode;

pub fn handle(root: &PathBuf, fallback: &Option<PathBuf>, request: &Request, configuration: &Configuration) -> Result<Response, HandlerError> {

    let path = Path::new(root).join(request.request_line.url.relative());

    if !path.starts_with(root) {
        return Err(HandlerError::new(StatusCode::Forbidden, "Location not valid"))
    }

    let file = match fs::read_to_string(&path) {
        Ok(file) => file,
        Err(_) => {
            match fallback {
                Some(fallback) => {
                    match fs::read_to_string(fallback) {
                        Ok(file) => file,
                        Err(_) => return Err(HandlerError::new(StatusCode::NotFound, "File not found"))
                    }
                },
                None => return Err(HandlerError::new(StatusCode::NotFound, "File not found"))
            }
        }
    };

    let media_type = match path.extension() {
        Some(extension) => {
            match extension.to_str() {
                Some("html") => MediaType::new(TopLevelMediaType::Text, "html"),
                Some("css") => MediaType::new(TopLevelMediaType::Text, "css"),
                Some("js") => MediaType::new(TopLevelMediaType::Text, "javascript"),
                Some("json") => MediaType::new(TopLevelMediaType::Application, "json"),
                _ => MediaType::new(TopLevelMediaType::Text, "plain")
            }
        }
        None => MediaType::new(TopLevelMediaType::Text, "plain")
    };

    let mut response_headers = Headers::new();
    response_headers.add("Content-Type", media_type.as_str());
    response_headers.add("Content-Length", file.len().to_string());

    Ok(Response::new(StatusCode::Ok, response_headers, file.into_bytes()))
}