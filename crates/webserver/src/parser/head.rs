use crate::configuration::configuration::Configuration;
use crate::http::headers::Headers;
use crate::http::request_line::RequestLine;
use crate::http::status_code::StatusCode;
use crate::parser::headers;
use crate::parser::parser_error::ParserError;
use crate::parser::request_line;

pub fn parse(head_buffer: Vec<u8>, configuration: &Configuration) -> Result<(RequestLine, Headers), ParserError> {
    let head = std::str::from_utf8(&*head_buffer).map_err(|_| ParserError::new(StatusCode::BadRequest, "Invalid UTF-8"))?;

    let mut head_lines = head.split("\r\n");

    let request_line = request_line::parse(head_lines.next().ok_or_else(|| ParserError::new(StatusCode::BadRequest, "Head malformed"))?)?;

    let headers = headers::parse(head_lines)?;

    Ok((request_line, headers))
}