use crate::http::headers::Headers;
use crate::parser::parser_error::ParserError;
use crate::http::status_code::StatusCode;

pub fn parse<'a>(raw_headers: impl Iterator<Item = &'a str>) -> Result<Headers, ParserError> {

    let mut headers = Headers::new();

    for raw_header in raw_headers {
        let (name, value) = raw_header
            .split_once(':')
            .ok_or_else(|| ParserError::new(StatusCode::BadRequest, "Malformed header"))?;

        headers.add(name.trim().to_string(), value.trim().to_string());
    }

    Ok(headers)
}


