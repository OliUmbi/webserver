use crate::http::request::Request;

pub fn matches(path: &String, request: &Request) -> bool {
    request.request_line.url.raw.starts_with(path)
}