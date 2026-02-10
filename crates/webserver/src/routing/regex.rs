use crate::http::request::Request;
use regex::Regex;
use crate::http::method::Method;

pub fn matches(regex: &Regex, methods: &Vec<Method>, request: &Request) -> bool {
    methods.contains(&request.request_line.method) && regex.is_match(request.request_line.url.raw.as_ref())
}
