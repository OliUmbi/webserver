use std::io::{BufReader, Read, Write};
use std::net::{SocketAddr, TcpStream};
use crate::http::response::Response;
use crate::server::server_error::ServerError;

pub struct Connection {
    reader: BufReader<TcpStream>,
    peer: SocketAddr
}
impl Connection {
    pub fn new(stream: TcpStream) -> Result<Self, ServerError> {
        let peer = stream.peer_addr().map_err(|_| ServerError::new("Failed to get peer address"))?;

        Ok(Self {
            reader: BufReader::new(stream),
            peer
        })
    }
    
    pub fn read(&mut self, buffer: &mut [u8]) -> Result<usize, ServerError> {
        self.reader.read(buffer).map_err(|_| ServerError::new("Failed to read connection"))
    }
    
    pub fn read_exact(&mut self, buffer: &mut [u8]) -> Result<(), ServerError> {
        self.reader.read_exact(buffer).map_err(|_| ServerError::new("Failed to read connection"))
    }
    
    pub fn write(&mut self, response: Response) -> Result<(), ServerError> {
        match self.reader.get_mut().write_all(&*response.to_http()) {
            Ok(_) => Ok(()),
            Err(_) => Err(ServerError::new("Failed to write response"))
        }
    }
}
