use std::io::{BufReader, Read};
use std::net::TcpStream;
use crate::http::headers::Headers;

pub fn parse_body(reader: &mut BufReader<&mut TcpStream>, mut already_read: Vec<u8>, headers: &Headers) -> Result<Vec<u8>, String> {

    already_read.drain(0..4);

    if let Some(te) = headers.transfer_encoding() {
        return if te.eq_ignore_ascii_case("chunked") {
            read_chunked_body(reader, already_read)
        } else {
            Err("Unsupported transfer-encoding".to_string())
        }
    }

    let content_length = headers.content_length().unwrap_or(0);

    if already_read.len() > content_length {
        already_read.truncate(content_length);
        return Ok(already_read);
    }

    let missing = content_length - already_read.len();
    let mut rest = vec![0u8; missing];

    match reader.read_exact(&mut rest) {
        Ok(_) => {
            already_read.extend(rest);
            Ok(already_read)
        }
        Err(_) => Err("Failed to read body".to_string())
    }
}

// todo rework (partly AI generated needs cleanup and restructuring)
fn read_chunked_body(reader: &mut BufReader<&mut TcpStream>, mut body: Vec<u8>) -> Result<Vec<u8>, String> {
    let mut result = Vec::new();

    loop {
        // --- read chunk size line ---
        let size_line = loop {
            if let Some(pos) = body.windows(2).position(|w| w == b"\r\n") {
                let line = body.drain(..pos + 2).collect::<Vec<u8>>();
                break String::from_utf8(line[..line.len() - 2].to_vec())
                    .map_err(|_| "invalid chunk size utf8")?;
            }

            let mut tmp = [0u8; 512];
            let n = reader.read(&mut tmp).map_err(|_| "read failed")?;
            if n == 0 {
                return Err("unexpected eof while reading chunk size".into());
            }
            body.extend_from_slice(&tmp[..n]);
        };

        let size = usize::from_str_radix(size_line.trim(), 16)
            .map_err(|_| "invalid chunk size")?;

        if size == 0 {
            // consume final CRLF
            while body.len() < 2 {
                let mut tmp = [0u8; 512];
                let n = reader.read(&mut tmp).map_err(|_| "read failed")?;
                if n == 0 {
                    return Err("unexpected eof after final chunk".into());
                }
                body.extend_from_slice(&tmp[..n]);
            }
            body.drain(..2);
            break;
        }

        // --- read chunk data ---
        while body.len() < size + 2 {
            let mut tmp = [0u8; 512];
            let n = reader.read(&mut tmp).map_err(|_| "read failed")?;
            if n == 0 {
                return Err("unexpected eof while reading chunk".into());
            }
            body.extend_from_slice(&tmp[..n]);
        }

        result.extend(body.drain(..size));
        body.drain(..2); // trailing CRLF
    }

    Ok(result)
}