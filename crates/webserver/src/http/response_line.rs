use crate::http::protocol::Protocol;
use crate::http::status_code::StatusCode;

#[derive(Debug)]
pub struct ResponseLine {
    pub protocol: Protocol,
    pub status: StatusCode,
}

impl ResponseLine {
    pub fn new(protocol: Protocol, status: StatusCode) -> ResponseLine {
        ResponseLine { protocol, status }
    }

    pub fn to_http(&self) -> String {
        format!("{} {}", self.protocol.to_string(), self.status.to_http())
    }
}
