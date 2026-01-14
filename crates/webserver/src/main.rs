use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use shared::{Headers, Method, Protocol, StatusCode, Url};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();

    for stream in listener.incoming() {
        println!("Incoming connection");

        handle_connection(stream.unwrap())
    }
}

// todo handle url, protocol, headers
#[derive(Debug)]
struct Request {
    method: Method,
    url: Url,
    protocol: Protocol,
    headers: Headers,
    body: String,
}

fn handle_connection(mut stream: TcpStream) {
    let mut reader = BufReader::new(&stream);

    let mut start = String::new();
    reader.read_line(&mut start).unwrap();

    let starts: Vec<_> = start.split_whitespace().collect();

    // todo handle incorrectly formatted headers
    if starts.len() != 3 {
        todo!()
    }

    // todo handle invalid method
    let method = Method::from_str(starts[0]);
    let url = Url::from_str(starts[1]);
    let protocol = Protocol::from_str(starts[2]);

    let mut headers = Headers::new();

    loop {
        let mut header = String::new();
        reader.read_line(&mut header).unwrap();

        if header.is_empty() || header == "\r\n" {
            break;
        }

        // todo handle error
        headers.add(header).unwrap();
    }

    let request = Request {
        method: method.unwrap(),
        url: url,
        protocol: protocol.unwrap(),
        headers: headers,
        body: "".to_string(),
    };

    println!("{:?}", request);

    let content = fs::read_to_string("C:/Users/olive/IdeaProjects/webserver/examples/simple/index.html").unwrap();

    let protocol = "HTTP/1.1";
    let status = format!("{} OK", StatusCode::Ok.code());

    let mut headers = Vec::new();
    headers.push("Content-Type: text/html; charset=utf-8".to_string());
    headers.push(format!("Content-Length: {}", content.len()));

    let mut head = Vec::new();
    head.push(format!("{} {}", protocol, status));
    head.append(&mut headers);

    let response = format!("{}\r\n\r\n{}", head.join("\r\n"), content);

    stream.write_all(response.as_bytes()).unwrap();
}
