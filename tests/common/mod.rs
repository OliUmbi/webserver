use std::collections::HashMap;

pub struct RequestResult {
    pub url: String,
    pub method: String,
    pub status: u8,
    pub headers: HashMap<String, String>,
    pub body: String
}

pub fn request(url: String, method: String) -> RequestResult {

    RequestResult {
        url: url,
        method: method,
        status: 200,
        headers: HashMap::from([
            ("Content-Type".to_string(), "application/json".to_string())
        ]),
        body: "Hello World!".to_string()
    }
}