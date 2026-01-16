use crate::{Headers, ResponseLine};

// todo metadata (ip, time)
pub struct Response {
    response_line: ResponseLine,
    headers: Headers,
    body: Vec<u8>
}