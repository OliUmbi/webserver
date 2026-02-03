use regex::Regex;

#[derive(Debug)]
pub enum Path {
    Exact(String),
    Prefix(String),
    Regex(Regex),
}
