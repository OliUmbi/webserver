use crate::http::response::Response;
use std::io::{Write};
use std::net::TcpStream;

pub enum Request {
    Raw(&'static str),
    Structured {
        method: &'static str,
        path: &'static str,
        version: &'static str,
        headers: Vec<&'static str>,
        body: &'static str,
    },
}

impl Request {    
    pub fn send(&self, address: String) -> Result<Response, String> {
        match self {
            Request::Raw(raw) => send_raw(address, raw.to_string()),
            Request::Structured {
                method,
                path,
                version,
                headers,
                body,
            } => send_structured(address, method, path, version, headers, body),
        }
    }
}

fn send_raw(address: String, raw: String) -> Result<Response, String> {
    let mut stream = TcpStream::connect(&address).map_err(|_| format!("Failed to connect to address: {}", address))?;

    stream.write_all(raw.as_bytes()).map_err(|_| format!("Failed to write request: {}", raw))?;

    Response::new(stream)
}

fn send_structured(
    address: String,
    method: &str,
    path: &str,
    version: &str,
    headers: &Vec<&str>,
    body: &str,
) -> Result<Response, String> {

    let mut raw = String::new();
    raw.push_str(method);
    raw.push(' ');
    raw.push_str(path);
    raw.push(' ');
    raw.push_str(version);
    raw.push_str("\r\n");

    for header in headers {
        raw.push_str(header);
        raw.push_str("\r\n");
    }

    raw.push_str("\r\n");
    raw.push_str(body);

    send_raw(address, raw)
}
