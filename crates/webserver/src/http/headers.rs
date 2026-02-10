use crate::http::status_code::StatusCode;
use crate::parser::parser_error::ParserError;
use regex::Regex;
use std::sync::LazyLock;

static NAME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[!#$%&'*+\-.^_`|~0-9A-Za-z]+$").unwrap());

static VALUE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[\t\x20-\x7E\x80-\xFF]*$").unwrap());

#[derive(Debug)]
pub struct Headers {
    entries: Vec<Header>,
}

#[derive(Debug)]
pub struct Header {
    name: String,
    value: String,
}

impl Headers {
    pub fn new() -> Self {
        Headers {
            entries: Vec::new(),
        }
    }

    pub fn add(
        &mut self,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), ParserError> {
        self.entries.push(Header::new(name, value)?);
        Ok(())
    }

    pub fn to_http(&self) -> String {
        self.entries
            .iter()
            .map(|header| format!("{}: {}", header.name, header.value))
            .collect::<Vec<String>>()
            .join("\r\n")
    }

    pub fn get(&self, name: &str) -> impl Iterator<Item = &str> {
        self.entries
            .iter()
            .filter(move |h| h.name.eq_ignore_ascii_case(name))
            .map(|h| h.value.as_str())
    }

    pub fn is_chunked(&self) -> Result<bool, ParserError> {
        let mut transfer_encodings = self.get("Transfer-Encoding");

        let transfer_encoding = transfer_encodings.next();

        if transfer_encodings.count() > 0 {
            return Err(ParserError::new(StatusCode::BadRequest, "Multiple Transfer-Encoding"));
        }

        Ok(transfer_encoding.map_or(false, |transfer_encoding| transfer_encoding.eq_ignore_ascii_case("chunked")))
    }

    pub fn content_length(&self) -> Result<Option<usize>, ParserError> {
        let mut content_lengths = self.get("Content-Length");

        let content_length = content_lengths.next();

        if content_lengths.count() > 0 {
            return Err(ParserError::new(StatusCode::BadRequest, "Multiple Content-Length"));
        }

        match content_length {
            Some(content_length) => content_length
                .parse::<usize>()
                .map(|content_length| Some(content_length))
                .map_err(|_| ParserError::new(StatusCode::BadRequest, "Failed to parse Content-Length")),
            None => Ok(None),
        }
    }

    pub fn expect(&self) -> bool {
        self.get("Expect").any(|expect| expect.eq_ignore_ascii_case("100-continue"))
    }
}

impl Header {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Result<Self, ParserError> {
        let name = name.into();
        if !NAME_REGEX.is_match(&name) {
            return Err(ParserError::new(
                StatusCode::BadRequest,
                "Invalid header name characters",
            ));
        }

        let value = value.into();
        if !VALUE_REGEX.is_match(&value) {
            return Err(ParserError::new(
                StatusCode::BadRequest,
                "Invalid header value characters",
            ));
        }

        Ok(Self { name, value })
    }
}
