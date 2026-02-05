use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

pub struct RawRequest {
    pub method: &'static str,
    pub path: &'static str,
    pub version: &'static str,
    pub headers: &'static [&'static str],
    pub body: &'static str,
}

pub fn send(base: &str, req: &RawRequest) -> (u16, bool) {
    let mut stream = TcpStream::connect(base).unwrap();
    stream.set_read_timeout(Some(Duration::from_secs(2))).unwrap();

    let mut data = String::new();
    data.push_str(req.method);
    data.push(' ');
    data.push_str(req.path);
    data.push(' ');
    data.push_str(req.version);
    data.push_str("\r\n");

    for h in req.headers {
        data.push_str(h);
        data.push_str("\r\n");
    }

    data.push_str("\r\n");
    data.push_str(req.body);

    stream.write_all(data.as_bytes()).unwrap();

    let mut buf = Vec::new();
    let closed = stream.read_to_end(&mut buf).is_ok();

    let text = std::str::from_utf8(&buf).unwrap();
    let status = text[9..12].parse().unwrap();

    (status, closed)
}

pub fn persistent_connection() -> anyhow::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:80")?;

    let req = "GET / HTTP/1.1\r\nHost: test\r\n\r\n";
    stream.write_all(req.as_bytes())?;
    stream.write_all(req.as_bytes())?;

    let mut buf = [0u8; 1024];
    let n = stream.read(&mut buf)?;

    assert!(n > 0);
    Ok(())
}

