use crate::http::body::Body;
use crate::http::headers::Headers;
use crate::http::request_line::RequestLine;

pub struct Request {
    pub request_line: RequestLine,
    pub headers: Headers,
    pub body: Body
}

impl Request {
    pub fn new(request_line: RequestLine, headers: Headers, body: Body) -> Request {
        Self {
            request_line,
            headers,
            body
        }
    }
}
