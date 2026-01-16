use shared::{Headers, RequestLine, Response, StatusCode};
use std::cmp::max;
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
    let response = handle_request(&stream);

    stream.write_all(response.to_http().as_bytes()).unwrap();
}

fn handle_request(stream: &TcpStream) -> Response {
    let head = read_head(stream).unwrap();

    let request_line = match RequestLine::new_from_http(head.0) {
        Ok(request_line) => request_line,
        Err(error) => return Response::error(StatusCode::BadRequest, error)
    };

    let headers = match Headers::new_from_http(head.1) {
        Ok(headers) => headers,
        Err(error) => return Response::error(StatusCode::BadRequest, error)
    };

    // todo handle body
    let body = match read_body(stream, head.2) {
        Ok(body) => body,
        Err(error) => return Response::error(StatusCode::BadRequest, error)
    };

    println!("{:?}", request_line);
    println!("{:?}", headers);
    println!("{:?}", body);

    let body = "Hello World";

    let mut response_headers = Headers::new();
    response_headers.add_from_str("Content-Type: text/plain").unwrap();
    response_headers.add_from_str(format!("Content-Length: {}", body.len()).as_str()).unwrap();

    Response::new(StatusCode::Ok, response_headers, body.to_string())
}

// todo rework bufreader to continue reading body
fn read_head(stream: &TcpStream) -> Result<(String, String, usize), String> {
    let mut reader = BufReader::new(stream);

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

    let (head_buffer, _) = head_buffer.split_at(head_end);

    match str::from_utf8(head_buffer) {
        Ok(head) => {
            match head.split_once("\r\n") {
                Some(components) => Ok((components.0.to_string(), components.1.to_string(), head_end + 4)),
                None => Err("Invalid head structure".to_string())
            }
        },
        Err(_) => Err("Failed to convert to UTF8".to_string())
    }
}

fn read_body(stream: &TcpStream, start: usize, length: u8) -> Result<Vec<u8>, String> {
    let mut reader = BufReader::new(stream);


}
