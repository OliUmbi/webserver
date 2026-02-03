use crate::http::request::Request;

pub fn matches(path: &String, request: &Request) -> bool {
    path.eq(&request.request_line.url.raw)
}
