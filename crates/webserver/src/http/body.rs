use crate::parser::parser_error::ParserError;
use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;
use crate::http::status_code::StatusCode;

pub struct Body<'a> {
    reader: &'a mut BufReader<&'a TcpStream>,
    buffer: Vec<u8>,
    kind: BodyKind,
    max_body_length: usize,
}

pub enum BodyKind {
    Fixed(usize),
    Chunked,
    Empty,
}

impl<'a> Body<'a> {
    pub fn new(reader: &'a mut BufReader<&'a TcpStream>, buffer: Vec<u8>, kind: BodyKind, max_body_length: usize) -> Self {
        Self {
            reader,
            buffer,
            kind,
            max_body_length
        }
    }

    pub fn read(&mut self) -> Result<Vec<u8>, ParserError> {
        match self.kind {
            BodyKind::Fixed(content_length) => self.read_fixed(content_length),
            BodyKind::Chunked => self.read_chunked(),
            BodyKind::Empty => Ok(Vec::new()),
        }
    }

    fn read_fixed(&mut self, content_length: usize) -> Result<Vec<u8>, ParserError> {
        if content_length > self.max_body_length {
            return Err(ParserError::new(StatusCode::ContentTooLarge, "Body too large"));
        }

        if self.buffer.len() > content_length {
            self.buffer.truncate(content_length);
        } else {
            let missing = content_length - self.buffer.len();
            let mut rest = vec![0u8; missing];

            self.reader.read_exact(&mut rest).map_err(|_| ParserError::new(StatusCode::BadRequest, "Failed to read body"))?;
            self.buffer.extend(rest);
        }

        Ok(self.buffer.clone())
    }

    // todo currently ignores already read bytes in buffer
    fn read_chunked(&mut self) -> Result<Vec<u8>, ParserError> {
        loop {
            let mut line = String::new();

            self.reader.read_line(&mut line).map_err(|_| ParserError::new(StatusCode::BadRequest, "failed to read chunk size"))?;
            let size = usize::from_str_radix(line.trim(), 16).map_err(|_| ParserError::new(StatusCode::BadRequest, "invalid chunk size"))?;

            if size == 0 {
                let mut crlf = [0u8; 2];
                self.reader.read_exact(&mut crlf).map_err(|_| ParserError::new(StatusCode::BadRequest, "expected CRLF"))?;
                break;
            }

            if self.buffer.len() + size > self.max_body_length {
                return Err(ParserError::new(StatusCode::ContentTooLarge, "Body too large"));
            }

            let mut chunk = vec![0u8; size];
            self.reader.read_exact(&mut chunk).map_err(|_| ParserError::new(StatusCode::BadRequest, "failed to read chunk"))?;
            self.buffer.extend(chunk);


            let mut crlf = [0u8; 2];
            self.reader.read_exact(&mut crlf).map_err(|_| ParserError::new(StatusCode::BadRequest, "expected CRLF after chunk"))?;
        }

        Ok(self.buffer.clone())
    }
}
