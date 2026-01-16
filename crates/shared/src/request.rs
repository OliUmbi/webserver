use crate::{Headers, RequestLine};

// todo metadata (ip, time)
pub struct Request {
    request_line: RequestLine,
    headers: Headers,
    body: Vec<u8>
}