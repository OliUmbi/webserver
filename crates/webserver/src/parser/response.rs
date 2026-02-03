use crate::configuration::internal::configuration::Configuration;
use crate::http::response::Response;
use crate::parser::parser_error::ParserError;
use crate::parser::{body, head, headers, message, response_line};
use crate::server::connection::Connection;

pub fn parse(
    connection: &mut Connection,
    configuration: &Configuration,
) -> Result<Response, ParserError> {
    let (head_buffer, body_buffer) = message::read(connection, configuration)?;

    let (raw_response_line, raw_headers) = head::parse(head_buffer)?;

    let response_line = response_line::parse(raw_response_line)?;
    let headers = headers::parse(raw_headers)?;
    let mut body = body::parse(body_buffer, &headers, configuration)?;
    body.read(connection)?;

    Ok(Response::new(response_line.status, headers, body.content()))
}
