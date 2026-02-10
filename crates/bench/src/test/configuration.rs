pub struct Configuration {
    address: String,
}

impl Configuration {
    pub fn new(address: impl Into<String>) -> Self {
        Self {
            address: address.into(),
        }
    }
    
    pub fn address(&self) -> String {
        self.address.clone()
    }
}
