use crate::configuration::configuration::Configuration;
use crate::http::request::Request;
use crate::http::status_code::StatusCode;
use crate::parser::parser_error::ParserError;
use crate::parser::{body, head};
use std::io::{BufReader, Read};
use std::net::TcpStream;

pub fn parse(stream: &TcpStream, configuration: &Configuration) -> Result<Request, ParserError> {
    let mut reader = BufReader::new(stream);

    let (head_buffer, body_buffer) = read(&mut reader, configuration)?;

    let (request_line, headers) = head::parse(head_buffer, configuration)?;
    
    let body = body::parse(&mut reader, body_buffer, &headers)?;

    Ok(Request::new(request_line, headers, body))
}

fn read(reader: &mut BufReader<&TcpStream>, configuration: &Configuration) -> Result<(Vec<u8>, Vec<u8>), ParserError> {

    let mut head_buffer = Vec::with_capacity(1024);

    loop {
        let mut temp_buffer = [0u8; 512];
        let read_bytes = reader.read(&mut temp_buffer).map_err(|_| ParserError::new(StatusCode::BadRequest, "Failed to read BufReader"))?;

        if read_bytes == 0 {
            break;
        }

        head_buffer.extend_from_slice(&temp_buffer[..read_bytes]);

        if let Some(head_end) = head_buffer[..].windows(4).position(|w| w == b"\r\n\r\n") {
            let (head_buffer, body_buffer) = head_buffer.split_at(head_end + 4);

            return Ok((head_buffer[..head_end].to_vec(), body_buffer.to_vec()));
        }

        if head_buffer.len() > configuration.server.limits.max_header_length {
            return Err(ParserError::new(StatusCode::RequestHeaderFieldsTooLarge, "Head too long"))
        }
    }

    Err(ParserError::new(StatusCode::BadRequest, "Head malformed"))
}
