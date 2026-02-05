use crate::http::method::Method;
use crate::http::protocol::Protocol;
use crate::http::request_line::RequestLine;
use crate::http::status_code::StatusCode;
use crate::http::url::Url;
use crate::parser::parser_error::ParserError;

pub fn parse(raw_request_line: String) -> Result<RequestLine, ParserError> {
    let mut components = raw_request_line.split(" ");

    let raw_method = components
        .next()
        .ok_or_else(|| ParserError::new(StatusCode::BadRequest, "Method missing"))?;

    let method = Method::from_str(raw_method)
        .map_err(|_| ParserError::new(StatusCode::BadRequest, "Invalid method"))?;

    let raw_url = components
        .next()
        .ok_or_else(|| ParserError::new(StatusCode::BadRequest, "Url missing"))?;

    let url = Url::from_str(raw_url);

    let raw_protocol = components
        .next()
        .ok_or_else(|| ParserError::new(StatusCode::BadRequest, "Protocol missing"))?;

    let protocol = Protocol::from_str(raw_protocol)
        .map_err(|_| ParserError::new(StatusCode::BadRequest, "Invalid protocol"))?;

    Ok(RequestLine::new(method, url, protocol))
}
