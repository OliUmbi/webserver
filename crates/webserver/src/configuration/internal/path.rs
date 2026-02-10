use crate::http::method::Method;
use regex::Regex;

#[derive(Debug)]
pub enum Path {
    Exact {
        exact: String,
        methods: Vec<Method>,
    },
    Prefix {
        prefix: String,
        methods: Vec<Method>,
    },
    Regex {
        regex: Regex,
        methods: Vec<Method>,
    },
}
