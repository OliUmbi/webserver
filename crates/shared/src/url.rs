#[derive(Debug)]
pub struct Url {
    raw: String,
}

impl Url {
    pub fn from_str(s: &str) -> Url {
        Url { raw: s.to_string() }
    }
}