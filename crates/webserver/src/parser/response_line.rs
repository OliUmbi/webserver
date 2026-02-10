use crate::http::protocol::Protocol;
use crate::http::response_line::ResponseLine;
use crate::http::status_code::StatusCode;
use crate::parser::parser_error::ParserError;

pub fn parse(raw_response_line: String) -> Result<ResponseLine, ParserError> {
    let mut components = raw_response_line.split(" ");

    let raw_protocol = components
        .next()
        .ok_or_else(|| ParserError::new(StatusCode::BadRequest, "Protocol missing"))?;

    let protocol = Protocol::from_str(raw_protocol)
        .map_err(|_| ParserError::new(StatusCode::BadRequest, "Unsupported Protocol"))?;

    let raw_status_code = components
        .next()
        .ok_or_else(|| ParserError::new(StatusCode::BadRequest, "Status missing"))?;

    let status_code = StatusCode::try_from(raw_status_code.to_string())
        .map_err(|_| ParserError::new(StatusCode::BadRequest, "Invalid status code"))?;

    Ok(ResponseLine::new(protocol, status_code))
}
