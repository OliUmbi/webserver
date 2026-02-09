use std::collections::HashMap;

pub struct Configuration {
    address: String,
    custom: HashMap<String, String>
}

impl Configuration {
    pub fn new(address: String, custom: HashMap<String, String>) -> Self {
        Self {
            address,
            custom
        }
    }
    
    pub fn address(&self) -> String {
        self.address.clone()
    }
    
    pub fn custom_string(&self, key: &str) -> String {
        self.custom.get(key).unwrap().clone()
    }

    pub fn custom_usize(&self, key: &str) -> usize {
        self.custom.get(key).map(|value| value.parse::<usize>().unwrap()).unwrap()
    }

    pub fn custom_bool(&self, key: &str) -> bool {
        self.custom.get(key).map(|value| value.parse::<bool>().unwrap()).unwrap()
    }
}
