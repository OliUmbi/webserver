// todo rework
#[derive(Debug)]
pub struct Url {
    pub raw: String,
}

impl Url {
    pub fn from_str(s: &str) -> Url {
        Url { raw: s.to_string() }
    }

    pub fn relative(&self) -> String {
        format!(".{}", self.raw)
    }
}