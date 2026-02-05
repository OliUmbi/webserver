use std::collections::HashMap;

#[derive(Debug)]
pub struct Headers {
    values: HashMap<String, String>,
}

impl Headers {
    pub fn new() -> Self {
        Headers {
            values: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.values.insert(name.into(), value.into());
    }

    pub fn to_http(&self) -> String {
        self.values
            .iter()
            .map(|header| format!("{}: {}", header.0, header.1))
            .collect::<Vec<String>>()
            .join("\r\n")
    }

    pub fn is_chunked(&self) -> bool {
        match self.values.get("Transfer-Encoding") {
            Some(transfer_encoding) => transfer_encoding.eq_ignore_ascii_case("chunked"),
            None => false,
        }
    }

    pub fn content_length(&self) -> Option<usize> {
        match self.values.get("Content-Length") {
            Some(length) => match length.parse::<usize>() {
                Ok(length) => Some(length),
                Err(_) => None,
            },
            None => None,
        }
    }
    
    pub fn host(&self) -> Option<&String> {
        self.values.get("Host")
    }
}
