use std::collections::HashMap;

// todo maybe add a HeaderName enum for known types
// todo multiple headers with same name? combine idk spec: https://www.w3.org/Protocols/rfc2616/rfc2616-sec4.html#sec4.2
#[derive(Debug)]
pub struct Headers {
    values: HashMap<String, String>,
}

impl Headers {
    pub fn new() -> Self {
        Headers { values: HashMap::new() }
    }

    pub fn new_from_http(s: String) -> Result<Self, String> {
        // todo review this maybe bad when no headers are specified

        let mut headers = Headers::new();

        for header in s.split("\r\n") {
            match headers.add_from_str(header) {
                Ok(_) => {}
                Err(_) => return Err("Header incorrectly formatted".to_string())
            }
        }

        Ok(headers)
    }
    
    pub fn add(&mut self, name: &str, value: &str) {
        self.values.insert(name.to_lowercase().to_string(), value.to_string());
    }

    pub fn add_from_str(&mut self, s: &str) -> Result<(), String> {
        let header = s.split_once(":");

        if header.is_none() {
            return Err(format!("invalid header: {}", s));
        }

        let name = header.unwrap().0.trim().to_lowercase().to_string();
        let value = header.unwrap().1.trim().to_string();

        if self.values.contains_key(&name) {
            return Err(format!("header already exists: {}", name));
        }

        self.values.insert(name, value);

        Ok(())
    }

    pub fn to_http(&self) -> String {
        self.values.iter().map(|header| format!("{}: {}", header.0, header.1)).collect::<Vec<String>>().join("\r\n")
    }

    pub fn content_length(&self) -> Option<usize> {
        match self.values.get("content-length") {
            Some(length) => {
                match length.parse::<usize>() {
                    Ok(length) => Some(length),
                    Err(_) => None
                }
            }
            None => None
        }
    }
}
