use crate::http::status_code::StatusCode;
use crate::parser::parser_error::ParserError;
use crate::server::connection::Connection;

pub struct Body {
    buffer: Vec<u8>,
    kind: BodyKind,
    max_body_length: usize,
}

pub enum BodyKind {
    Fixed(usize),
    Chunked,
    Empty,
}

impl Body {
    pub fn new(buffer: Vec<u8>, kind: BodyKind, max_body_length: usize) -> Self {
        Self {
            buffer,
            kind,
            max_body_length
        }
    }

    pub fn read(&mut self, connection: &mut Connection) -> Result<Vec<u8>, ParserError> {
        match self.kind {
            BodyKind::Fixed(content_length) => self.read_fixed(connection, content_length),
            BodyKind::Chunked => self.read_chunked(connection),
            BodyKind::Empty => Ok(Vec::new()),
        }
    }

    fn read_fixed(&mut self, connection: &mut Connection, content_length: usize) -> Result<Vec<u8>, ParserError> {
        if content_length > self.max_body_length {
            return Err(ParserError::new(StatusCode::ContentTooLarge, "Body too large"));
        }

        if self.buffer.len() > content_length {
            self.buffer.truncate(content_length);
        } else {
            let missing = content_length - self.buffer.len();
            let mut rest = vec![0u8; missing];

            connection.read_exact(&mut rest).map_err(|_| ParserError::new(StatusCode::BadRequest, "Failed to read body"))?;
            self.buffer.extend(rest);
        }

        Ok(self.buffer.clone())
    }

    // todo note already read bytes in buffer
    fn read_chunked(&mut self, connection: &mut Connection) -> Result<Vec<u8>, ParserError> {
        todo!()
    }
}
