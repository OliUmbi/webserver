use std::cmp::max;
use std::io::{BufReader, Read};
use std::net::TcpStream;

const MAX_HEADER_LENGTH: usize = 8 * 1024; // todo move


// todo delete this file

pub fn parse_head(reader: &mut BufReader<&mut TcpStream>) -> Result<(String, String, Vec<u8>), String> {

    let mut head_buffer = Vec::with_capacity(1024);
    let mut scanned = 0;

    loop {
        let mut temp_buffer = [0u8; 512];
        let read_bytes = match reader.read(&mut temp_buffer) {
            Ok(bytes) => bytes,
            Err(_) => return Err("Failed to read BufReader".to_string())
        };

        if read_bytes == 0 {
            break;
        }

        head_buffer.extend_from_slice(&temp_buffer[..read_bytes]);

        if head_buffer[scanned..].windows(4).any(|w| w == b"\r\n\r\n") {
            break;
        }

        scanned = max(scanned + read_bytes - 4, 0);

        if head_buffer.len() > MAX_HEADER_LENGTH {
            return Err("Too long".to_string())
        }
    }

    let head_end = head_buffer
        .windows(4)
        .position(|w| w == b"\r\n\r\n")
        .unwrap();

    let (head_buffer, body_buffer) = head_buffer.split_at(head_end);

    match str::from_utf8(head_buffer) {
        Ok(head) => {
            match head.split_once("\r\n") {
                Some(components) => Ok((components.0.to_string(), components.1.to_string(), body_buffer.to_vec())),
                None => Err("Invalid head structure".to_string())
            }
        },
        Err(_) => Err("Failed to convert to UTF8".to_string())
    }
}