use crate::configuration::internal::configuration::Configuration;
use crate::http::request::Request;
use crate::parser::parser_error::ParserError;
use crate::parser::{body, head, headers, message, request_line};
use crate::server::connection::Connection;

pub fn parse(
    connection: &mut Connection,
    configuration: &Configuration,
) -> Result<Request, ParserError> {
    let (head_buffer, body_buffer) = message::read(connection, configuration)?;

    let (raw_request_line, raw_headers) = head::parse(head_buffer)?;

    let request_line = request_line::parse(raw_request_line)?;
    let headers = headers::parse(raw_headers)?;
    let body = body::parse(body_buffer, &headers, configuration)?;

    Ok(Request::new(request_line, headers, body))
}
