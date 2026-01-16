use shared::{Headers, Method, Protocol, RequestLine, Url};
use std::cmp::max;
use std::io::{BufReader, Read};
use std::net::{TcpListener, TcpStream};

const MAX_HEADER_LENGTH: usize = 8 * 1024;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();

    for stream in listener.incoming() {
        println!("Incoming connection");

        handle_connection(stream.unwrap())
    }
}

fn handle_connection(stream: TcpStream) {
    let head = read_head(&stream).unwrap();

    let request_line = parse_request_line(&head).unwrap();

    // todo check request line

    let headers = parse_headers(&head).unwrap();

    println!("{}", head);
    println!("{:?}", request_line);
    println!("{:?}", headers);

    // todo check headers

    // todo handle body
}


fn read_head(mut stream: &TcpStream) -> Result<String, String> {
    let mut reader = BufReader::new(&mut stream);

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
        Ok(head) => Ok(head.to_string()),
        Err(_) => Err("Failed to convert to UTF8".to_string())
    }
}

fn parse_request_line(head: &String) -> Result<RequestLine, String> {
    // todo review this maybe bad when no headers are specified
    let (request_line, _) = head.split_once("\r\n").unwrap();

    let mut request_line_components = request_line.split(" ");

    let method = match request_line_components.next() {
        Some(value) => {
            match Method::from_str(value) {
                Some(method) => method,
                None => return Err("Invalid method".to_string())
            }
        },
        None => return Err("Request line invalid, method missing".to_string())
    };

    let url = match request_line_components.next() {
        Some(value) => Url::from_str(value),
        None => return Err("Request line invalid, url missing".to_string())
    };

    let protocol = match request_line_components.next() {
        Some(value) => {
            match Protocol::from_str(value) {
                Some(protocol) => protocol,
                None => return Err("Invalid protocol".to_string())
            }
        },
        None => return Err("Request line invalid, protocol missing".to_string())
    };

    Ok(RequestLine::new(method, url, protocol))
}

fn parse_headers(head: &String) -> Result<Headers, String> {
    // todo review this maybe bad when no headers are specified
    let (_, head_headers) = head.split_once("\r\n").unwrap();

    let mut headers = Headers::new();

    for header in head_headers.split("\r\n") {
        match headers.add(header) {
            Ok(_) => {}
            Err(_) => return Err("Header incorrectly formatted".to_string())
        }
    }

    Ok(headers)
}
