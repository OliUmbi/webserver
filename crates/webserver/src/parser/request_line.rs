use crate::parser::parser_error::ParserError;
use crate::http::method::Method;
use crate::http::protocol::Protocol;
use crate::http::request_line::RequestLine;
use crate::http::status_code::StatusCode;
use crate::http::url::Url;

pub fn parse(raw_request_line: &str) -> Result<RequestLine, ParserError> {
    // todo review this maybe bad when no headers are specified

    let mut components = raw_request_line.split(" ");

    let method = match components.next() {
        Some(value) => match Method::from_str(value) {
            Some(method) => method,
            None => return Err(ParserError::new(StatusCode::BadRequest, format!("Invalid method: {}", value))),
        },
        None => return Err(ParserError::new(StatusCode::BadRequest, "Request line invalid, method missing"))
    };

    let url = match components.next() {
        Some(value) => Url::from_str(value),
        None => return Err(ParserError::new(StatusCode::BadRequest, "Request line invalid, url missing"))
    };

    let protocol = match components.next() {
        Some(value) => {
            match Protocol::from_str(value) {
                Some(protocol) => protocol,
                None => return Err(ParserError::new(StatusCode::BadRequest, "Invalid protocol"))
            }
        },
        None => return Err(ParserError::new(StatusCode::BadRequest, "Request line invalid, protocol missing".to_string()))
    };

    Ok(RequestLine::new(method, url, protocol))
}
