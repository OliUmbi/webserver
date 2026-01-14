use std::collections::HashMap;

#[derive(Debug)]
pub struct Headers {
    values: HashMap<String, String>,
}

impl Headers {
    pub fn new() -> Headers {
        Headers { values: HashMap::new() }
    }

    pub fn add(&mut self, s: String) -> Result<(), String> {
        let header = s.split_once(":");

        if header.is_none() {
            return Err(format!("invalid header: {}", s));
        }

        let name = header.unwrap().0.trim().to_string();
        let value = header.unwrap().1.trim().to_string();

        if self.values.contains_key(&name) {
            return Err(format!("header already exists: {}", name));
        }

        self.values.insert(name, value);

        Ok(())
    }
}
