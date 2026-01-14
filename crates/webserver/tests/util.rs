use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

pub fn request(url: &str, method: &str, protocol: &str, headers: Vec<String>, body: &str) -> u16 {

    let mut stream = TcpStream::connect("127.0.0.1:80").unwrap();

    stream.write_all(format!("{} {} {}\r\n{}\r\n\r\n{}", method, url, protocol, headers.join("\r\n"), body).as_bytes()).unwrap();

    let reader = BufReader::new(stream);

    let response: Vec<_> = reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status = response.first().unwrap().parse::<String>().unwrap()[9..12].parse::<u16>().unwrap();

    println!("Request: {response:#?}");

    status
}