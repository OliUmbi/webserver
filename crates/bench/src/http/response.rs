use std::io::Read;
use std::net::TcpStream;

pub struct Response {
    stream: TcpStream,
    data: String,
    pub status_code: usize
}

impl Response {
    pub fn new(mut stream: TcpStream) -> Result<Self, String> {
        let mut buffer = [0u8; 16];
        let n = stream.read(&mut buffer).map_err(|_| "Failed to read response")?;

        if n == 0 {
            return Err("Failed to read response".to_string())
        }

        let data = String::from_utf8(Vec::from(buffer)).map_err(|_| "Failed to interpret response to UTF-8")?;

        let status_code = data[9..12].parse().map_err(|_| format!("Failed to read status code: {}", data))?;

        Ok(Self {
            stream,
            data,
            status_code
        })
    }
}
