use shared::{Headers, RequestLine, Response, StatusCode};
use std::cmp::max;
use std::fs;
use std::io::{BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};

const MAX_HEADER_LENGTH: usize = 8 * 1024;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();

    for stream in listener.incoming() {
        println!("Incoming connection");

        handle_connection(&mut stream.unwrap())
    }
}

fn handle_connection(stream: &mut TcpStream) {
    let response = handle_request(stream);

    stream.write_all(response.to_http().as_bytes()).unwrap();
}

fn handle_request(stream: &mut TcpStream) -> Response {

    let mut reader = BufReader::new(stream);

    let (raw_request_line, raw_headers, body_already_read) = match read_head(&mut reader) {
        Ok(head) => head,
        Err(error) => return Response::error(StatusCode::BadRequest, error)
    };

    let request_line = match RequestLine::new_from_http(raw_request_line) {
        Ok(request_line) => request_line,
        Err(error) => return Response::error(StatusCode::BadRequest, error)
    };

    let headers = match Headers::new_from_http(raw_headers) {
        Ok(headers) => headers,
        Err(error) => return Response::error(StatusCode::BadRequest, error)
    };

    // todo handle body
    let body = match read_body(&mut reader, body_already_read, &headers) {
        Ok(body) => body,
        Err(error) => return Response::error(StatusCode::BadRequest, error)
    };

    println!("{:?}", request_line);
    println!("{:?}", headers);
    println!("{:?}", str::from_utf8(body.as_slice()).unwrap());

    let body = fs::read_to_string("C:/Users/olive/IdeaProjects/webserver/examples/simple/index.html").unwrap();

    let mut response_headers = Headers::new();
    response_headers.add_from_str("Content-Type: text/html").unwrap();
    response_headers.add_from_str(format!("Content-Length: {}", body.len()).as_str()).unwrap();

    Response::new(StatusCode::Ok, response_headers, body.to_string())
}

// todo rework bufreader to continue reading body
fn read_head(reader: &mut BufReader<&mut TcpStream>) -> Result<(String, String, Vec<u8>), String> {

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

fn read_body(reader: &mut BufReader<&mut TcpStream>, mut already_read: Vec<u8>, headers: &Headers) -> Result<Vec<u8>, String> {

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

