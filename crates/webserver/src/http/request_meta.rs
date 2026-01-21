use std::time::Instant;

// todo metadata (ip, time)
pub struct RequestMeta {
    ip: String,
    time: Instant,
}