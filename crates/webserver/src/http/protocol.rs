#[derive(Debug)]
pub enum Protocol {
    Http1_1,
}

impl Protocol {
    pub fn from_str(s: &str) -> Result<Self, ()> {
        match s.to_uppercase().as_str() {
            "HTTP/1.1" => Ok(Protocol::Http1_1),
            _ => Err(()),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Protocol::Http1_1 => "HTTP/1.1".to_string(),
        }
    }
}
