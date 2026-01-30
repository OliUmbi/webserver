use regex::bytes::Regex;
use crate::http::request::Request;

pub fn matches(path: &String, request: &Request) -> bool {
    let regex = Regex::new(path).unwrap();

    regex.is_match(request.request_line.url.raw.as_ref())
}