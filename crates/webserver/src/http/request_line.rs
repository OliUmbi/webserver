use crate::http::method::Method;
use crate::http::protocol::Protocol;
use crate::http::url::Url;

#[derive(Debug)]
pub struct RequestLine {
    pub method: Method,
    pub url: Url,
    pub protocol: Protocol
}

impl RequestLine {
    pub fn new(method: Method, url: Url, protocol: Protocol) -> RequestLine {
        RequestLine {
            method,
            url,
            protocol
        }
    }
}
