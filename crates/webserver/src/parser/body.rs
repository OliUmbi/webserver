use crate::http::body::{Body, BodyKind};
use crate::http::headers::Headers;
use crate::parser::parser_error::ParserError;
use std::io::BufReader;
use std::net::TcpStream;
use crate::configuration::configuration::Configuration;
use crate::http::status_code::StatusCode;

pub fn parse<'a>(reader: &'a mut BufReader<&'a TcpStream>, body_buffer: Vec<u8>, headers: &Headers, configuration: &Configuration) -> Result<Body<'a>, ParserError> {
    let mut body_kind = BodyKind::Empty;

    if headers.is_chunked() {
        body_kind = BodyKind::Chunked;
    }

    if let Some(content_length) = headers.content_length() {
        if content_length > configuration.server.limits.max_body_length { 
            return Err(ParserError::new(StatusCode::ContentTooLarge, "Body too large"));
        }
        body_kind = BodyKind::Fixed(content_length);
    }

    Ok(Body::new(reader, body_buffer, body_kind, configuration.server.limits.max_body_length))
}
