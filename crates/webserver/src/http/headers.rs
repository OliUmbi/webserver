use std::collections::HashMap;

#[derive(Debug)]
pub struct Headers {
    entries: Vec<Header>,
}

#[derive(Debug)]
pub struct Header {
    name: String,
    value: String,
}

impl Headers {
    pub fn new() -> Self {
        Headers {
            entries: Vec::new(),
        }
    }

    pub fn add(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.entries.push(Header::new(name, value));
    }

    pub fn to_http(&self) -> String {
        self.entries
            .iter()
            .map(|header| format!("{}: {}", header.name, header.value))
            .collect::<Vec<String>>()
            .join("\r\n")
    }

    pub fn get(&self, name: &str) -> impl Iterator<Item = &str> {
        self.entries
            .iter()
            .filter(move |h| h.name.eq_ignore_ascii_case(name))
            .map(|h| h.value.as_str())
    }

    pub fn is_chunked(&self) -> bool {
        self.get("Transfer-Encoding")
            .any(|transfer_encoding| transfer_encoding.eq_ignore_ascii_case("chunked"))
    }

    pub fn content_length(&self) -> Option<usize> {
        self.get("Content-Length")
            .next()
            .and_then(|len| len.parse().ok())
    }
}

impl Header {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}
