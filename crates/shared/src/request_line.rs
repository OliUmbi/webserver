use crate::{Method, Protocol, Url};

#[derive(Debug)]
pub struct RequestLine {
    method: Method,
    url: Url,
    protocol: Protocol
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
