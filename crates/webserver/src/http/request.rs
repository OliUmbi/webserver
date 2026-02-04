use crate::http::body::Body;
use crate::http::headers::Headers;
use crate::http::request_line::RequestLine;

pub struct Request {
    pub request_line: RequestLine,
    pub headers: Headers,
    pub body: Body,
}

impl Request {
    pub fn new(request_line: RequestLine, headers: Headers, body: Body) -> Request {
        Self {
            request_line,
            headers,
            body,
        }
    }

    pub fn to_http(&self) -> Vec<u8> {
        let mut message = Vec::new();
        message.extend_from_slice(self.request_line.to_http().as_bytes());
        message.extend_from_slice("\r\n".as_bytes());
        message.extend_from_slice(self.headers.to_http().as_bytes());
        message.extend_from_slice("\r\n\r\n".as_bytes());
        message.extend_from_slice(&*self.body.content());

        message
    }
}

