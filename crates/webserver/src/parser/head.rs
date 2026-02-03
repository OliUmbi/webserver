use crate::http::status_code::StatusCode;
use crate::parser::parser_error::ParserError;

pub fn parse(head_buffer: Vec<u8>) -> Result<(String, Vec<String>), ParserError> {
    let head = String::from_utf8(head_buffer)
        .map_err(|_| ParserError::new(StatusCode::BadRequest, "Invalid UTF-8"))?;

    let mut lines = head.split("\r\n");

    let first = lines
        .next()
        .ok_or_else(|| ParserError::new(StatusCode::BadRequest, "Head malformed"))?
        .to_string();

    let rest = lines.map(str::to_string).collect();

    Ok((first, rest))
}
