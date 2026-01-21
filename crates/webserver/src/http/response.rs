use crate::http::headers::Headers;
use crate::http::protocol::Protocol;
use crate::http::response_line::ResponseLine;
use crate::http::status_code::StatusCode;

// todo metadata (ip, time)
pub struct Response {
    response_line: ResponseLine,
    headers: Headers,
    body: String
}

impl Response {
    pub fn new(status: StatusCode, headers: Headers, body: String) -> Self {
        Response {
            response_line: ResponseLine::new(Protocol::Http1_1, status),
            headers,
            body
        }
    }
    
    pub fn error(status: StatusCode, message: String) -> Self {

        let body = format!("Error: {}", message);
        
        let mut headers = Headers::new();
        headers.add("Content-Type", "text/plain");
        headers.add("Content-Length", format!("{}", body.len()).as_str());
        
        Response {
            response_line: ResponseLine::new(Protocol::Http1_1, status),
            headers,
            body            
        }
    }

    pub fn to_http(&self) -> String {
        format!("{}\r\n{}\r\n\r\n{}", self.response_line.to_http(), self.headers.to_http(), self.body)
    }
}
