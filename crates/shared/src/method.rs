#[derive(Debug)]
pub enum Method {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Other(String)
}

impl Method {
    pub fn as_str(&self) -> &str {
        match self {
            Method::Get => "GET",
            Method::Head => "HEAD",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
            Method::Connect => "CONNECT",
            Method::Options => "OPTIONS",
            Method::Trace => "TRACE",
            Method::Other(other) => other.as_str()
        }
    }

    // todo maybe change to Result
    pub fn from_str(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "GET" => Method::Get,
            "HEAD" => Method::Head,
            "POST" => Method::Post,
            "PUT" => Method::Put,
            "DELETE" => Method::Delete,
            "CONNECT" => Method::Connect,
            "OPTIONS" => Method::Options,
            "TRACE" => Method::Trace,
            other => Method::Other(other.to_string())
        }
    }
}
