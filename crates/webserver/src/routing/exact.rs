use crate::http::method::Method;
use crate::http::request::Request;

pub fn matches(path: &String, methods: &Vec<Method>, request: &Request) -> bool {
    methods.contains(&request.request_line.method) && path.eq(&request.request_line.url.raw)
}
