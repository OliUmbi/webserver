use std::io::Read;
use std::net::TcpStream;

pub struct Response {
    stream: TcpStream,
    data: String
}

impl Response {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            data: String::new()
        }
    }

    pub fn status_code(&mut self) -> usize {
        if self.data.len() < 12 {
            let mut buffer = [0u8; 16];
            let _ = self.stream.read(&mut buffer).unwrap();

            self.data.push_str(String::from_utf8(Vec::from(buffer)).unwrap().as_str());
        }

        self.data[9..12].parse().unwrap()
    }
}
