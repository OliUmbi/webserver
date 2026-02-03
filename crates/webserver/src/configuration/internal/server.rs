use std::time::Duration;

#[derive(Debug)]
pub struct Server {
    pub threads: usize,
    pub connections: usize,
    pub port: usize,
    pub timeout: Duration,
    pub max_header_length: usize,
    pub max_body_length: usize,
}
