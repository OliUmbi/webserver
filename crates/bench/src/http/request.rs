use crate::http::response::Response;
use std::io::{Write};
use std::net::TcpStream;

pub enum Request {
    Raw {
        address: String,
        raw: String,
    },
    Structured {
        address: String,
        method: &'static str,
        path: &'static str,
        version: &'static str,
        headers: Vec<&'static str>,
        body: &'static str,
    },
}

impl Request {
    pub fn send(&self) -> Response {
        match self {
            Request::Raw { address, raw } => send_raw(address, raw),
            Request::Structured {
                address,
                method,
                path,
                version,
                headers,
                body,
            } => send_structured(address, method, path, version, headers, body),
        }
    }
}

fn send_raw(address: &String, raw: &String) -> Response {
    let mut stream = TcpStream::connect(address).unwrap();

    stream.write_all(raw.as_bytes()).unwrap();

    Response::new(stream)
}

fn send_structured(
    address: &String,
    method: &str,
    path: &str,
    version: &str,
    headers: &Vec<&str>,
    body: &str,
) -> Response {
    let mut stream = TcpStream::connect(address).unwrap();

    let mut message = String::new();
    message.push_str(method);
    message.push(' ');
    message.push_str(path);
    message.push(' ');
    message.push_str(version);
    message.push_str("\r\n");

    for header in headers {
        message.push_str(header);
        message.push_str("\r\n");
    }

    message.push_str("\r\n");
    message.push_str(body);

    stream.write_all(message.as_bytes()).unwrap();

    Response::new(stream)
}
