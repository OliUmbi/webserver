use crate::{Protocol, StatusCode};

#[derive(Debug)]
pub struct ResponseLine {
    protocol: Protocol,
    status: StatusCode
}

impl ResponseLine {
    pub fn new(protocol: Protocol, status: StatusCode) -> ResponseLine {
        ResponseLine {
            protocol,
            status
        }
    }
}
