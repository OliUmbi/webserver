use crate::http::request::Request;
use crate::http::response::Response;
use crate::server::server_error::ServerError;
use std::io::{Read, Write};
use std::net::{Shutdown, SocketAddr, TcpStream};
use std::time::Duration;

pub struct Connection {
    stream: TcpStream,
    peer: SocketAddr,
}
impl Connection {
    pub fn new(stream: TcpStream, timeout: Duration) -> Result<Self, ServerError> {
        stream
            .set_read_timeout(Some(timeout))
            .map_err(|_| ServerError::new("Failed to set timeout"))?;

        stream
            .set_write_timeout(Some(timeout))
            .map_err(|_| ServerError::new("Failed to set timeout"))?;

        let peer = stream
            .peer_addr()
            .map_err(|_| ServerError::new("Failed to get peer address"))?;

        Ok(Self { stream, peer })
    }

    pub fn read(&mut self, buffer: &mut [u8]) -> Result<usize, ServerError> {
        self.stream
            .read(buffer)
            .map_err(|_| ServerError::new("Failed to read connection"))
    }

    pub fn read_exact(&mut self, buffer: &mut [u8]) -> Result<(), ServerError> {
        self.stream
            .read_exact(buffer)
            .map_err(|_| ServerError::new("Failed to read connection"))
    }

    pub fn write_response(&mut self, response: Response) -> Result<(), ServerError> {
        self.stream
            .write_all(&*response.to_http())
            .map_err(|_| ServerError::new("Failed to write response"))?;

        self.stream
            .flush()
            .map_err(|_| ServerError::new("Failed to write response"))?;

        Ok(())
    }

    pub fn write_request(&mut self, request: &mut Request) -> Result<(), ServerError> {
        self.stream
            .write_all(&*request.to_http())
            .map_err(|_| ServerError::new("Failed to write request"))?;

        self.stream
            .flush()
            .map_err(|_| ServerError::new("Failed to write request"))?;

        Ok(())
    }

    pub fn close(&mut self) -> Result<(), ServerError> {
        self.stream
            .shutdown(Shutdown::Both)
            .map_err(|_| ServerError::new("Failed to close connection"))
    }
}
