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

    pub fn new_from_http(s: String) -> Result<RequestLine, String> {
        // todo review this maybe bad when no headers are specified

        let mut components = s.split(" ");

        let method = match components.next() {
            Some(value) => Method::from_str(value),
            None => return Err("Request line invalid, method missing".to_string())
        };

        let url = match components.next() {
            Some(value) => Url::from_str(value),
            None => return Err("Request line invalid, url missing".to_string())
        };

        let protocol = match components.next() {
            Some(value) => {
                match Protocol::from_str(value) {
                    Some(protocol) => protocol,
                    None => return Err("Invalid protocol".to_string())
                }
            },
            None => return Err("Request line invalid, protocol missing".to_string())
        };

        Ok(RequestLine::new(method, url, protocol))
    }
}
