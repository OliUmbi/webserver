use crate::http::headers::Headers;
use crate::http::status_code::StatusCode;
use crate::parser::parser_error::ParserError;

pub fn parse(raw_headers: Vec<String>) -> Result<Headers, ParserError> {
    let mut headers = Headers::new();

    for raw_header in raw_headers {
        let (name, value) = raw_header
            .split_once(':')
            .ok_or_else(|| ParserError::new(StatusCode::BadRequest, "Malformed header"))?;

        headers.add(name.trim(), value.trim())?;
    }

    Ok(headers)
}
