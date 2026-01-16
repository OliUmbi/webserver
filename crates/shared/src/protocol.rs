#[derive(Debug)]
pub enum Protocol {
    Http1_1
}

impl Protocol {
    pub fn from_str(s: &str) -> Option<Protocol> {
        match s.to_uppercase().as_str() {
            "HTTP/1.1" => Some(Protocol::Http1_1),
            _ => None
        }
    }

    pub fn to_http(&self) -> String {
        match self {
            Protocol::Http1_1 => "HTTP/1.1".to_string()
        }
    }
}
