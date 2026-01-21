use std::{fs, thread};
use std::io::{BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{mpsc, Arc, Mutex};
use crate::http::headers::Headers;
use crate::http::parser::body::parse_body;
use crate::http::parser::request_head::parse_head;
use crate::http::request_line::RequestLine;
use crate::http::response::Response;
use crate::http::status_code::StatusCode;

const THREADS: usize = 4;

pub fn run() {
    // todo implement cap / backpressure
    let (sender, receiver) = mpsc::sync_channel(100);
    let receiver = Arc::new(Mutex::new(receiver));

    for _ in 0..THREADS {
        let receiver = Arc::clone(&receiver);
        thread::spawn(move || {
            loop {
                let stream = receiver.lock().unwrap().recv().unwrap();
                handle_connection(stream);
            }
        });
    }

    // todo handle not being able to bind to the port
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();

    // todo stream is only a connection attempt and not established so there should be checks
    for stream in listener.incoming() {
        // blocks here if queue is full -> backpressure
        sender.send(stream.unwrap()).unwrap();
    }
}

fn handle_connection(stream: &mut TcpStream) {
    let response = handle_request(stream);

    stream.write_all(response.to_http().as_bytes()).unwrap();
}

fn handle_request(stream: &mut TcpStream) -> Response {

    let mut reader = BufReader::new(stream);

    let (raw_request_line, raw_headers, body_already_read) = match parse_head(&mut reader) {
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
    let body = match parse_body(&mut reader, body_already_read, &headers) {
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
