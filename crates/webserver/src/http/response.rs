use crate::http::headers::Headers;
use crate::http::protocol::Protocol;
use crate::http::response_line::ResponseLine;
use crate::http::status_code::StatusCode;

// todo metadata (ip, time)
pub struct Response {
    response_line: ResponseLine,
    headers: Headers,
    body: Vec<u8>
}

impl Response {
    pub fn new(status: StatusCode, headers: Headers, body: Vec<u8>) -> Self {
        Response {
            response_line: ResponseLine::new(Protocol::Http1_1, status),
            headers,
            body
        }
    }
    
    // todo implement a content-length system that works for multiple use cases
    
    // todo rework
    pub fn error(status: StatusCode, message: String) -> Self {

        let body = format!("Error: {}", message);
        
        let mut headers = Headers::new();
        headers.add("Content-Type", "text/plain");
        headers.add("Content-Length", format!("{}", body.len()));
        
        Response {
            response_line: ResponseLine::new(Protocol::Http1_1, status),
            headers,
            body: body.into_bytes()
        }
    }

    // todo review
    pub fn to_http(&self) -> Vec<u8> {
        let mut message = Vec::new();
        message.extend_from_slice(self.response_line.to_http().as_bytes());
        message.extend_from_slice("\r\n".as_bytes());
        message.extend_from_slice(self.headers.to_http().as_bytes());
        message.extend_from_slice("\r\n\r\n".as_bytes());
        message.extend_from_slice(&*self.body);

        message
    }
}
