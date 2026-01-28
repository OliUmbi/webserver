use crate::http::body::Body;
use crate::http::headers::Headers;
use crate::http::request_line::RequestLine;

// todo metadata (ip, time)
pub struct Request {
    request_line: RequestLine,
    headers: Headers,
    body: Body
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
