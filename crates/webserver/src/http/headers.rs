use std::collections::HashMap;

// todo maybe add a HeaderName enum for known types
// todo multiple headers with same name? combine idk spec: https://www.w3.org/Protocols/rfc2616/rfc2616-sec4.html#sec4.2
// todo review current lowercase headers saving
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

    pub fn add(&mut self, name: String, value: String) {
        self.values.insert(name, value);
    }

    // todo move to composer
    pub fn to_http(&self) -> String {
        self.values
            .iter()
            .map(|header| format!("{}: {}", header.0, header.1))
            .collect::<Vec<String>>()
            .join("\r\n")
    }

    pub fn content_length(&self) -> Option<usize> {
        match self.values.get("content-length") {
            Some(length) => match length.parse::<usize>() {
                Ok(length) => Some(length),
                Err(_) => None,
            },
            None => None,
        }
    }

    pub fn transfer_encoding(&self) -> Option<String> {
        self.values.get("transfer-encoding").cloned()
    }
}
