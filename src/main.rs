use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();

    for stream in listener.incoming() {
        println!("Incoming connection");

        handle_connection(stream.unwrap())
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);

    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {http_request:#?}");

    let content = fs::read_to_string("C:/Users/olive/IdeaProjects/webserver/examples/simple/index.html").unwrap();

    let protocol = "HTTP/1.1";
    let status = "200 OK";

    let mut headers = Vec::new();
    headers.push("Content-Type: text/html; charset=utf-8".to_string());
    headers.push(format!("Content-Length: {}", content.len()));

    let mut head = Vec::new();
    head.push(format!("{} {}", protocol, status));
    head.append(&mut headers);

    let response = format!("{}\r\n\r\n{}", head.join("\r\n"), content);

    stream.write_all(response.as_bytes()).unwrap();
}
