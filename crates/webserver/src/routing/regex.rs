use crate::http::request::Request;
use regex::Regex;

pub fn matches(regex: &Regex, request: &Request) -> bool {
    regex.is_match(request.request_line.url.raw.as_ref())
}
