use std::thread::sleep;
use std::time::Duration;

mod setup;
mod util;

#[test]
fn correct() {
    assert_eq!(200, util::request("/", "GET", "HTTP/1.1", vec!["Host: Yeet".to_string()], ""));
}

#[test]
fn method_missing() {
    assert_eq!(400, util::request("/", "", "HTTP/1.1", vec!["Host: Yeet".to_string()], ""));
}

#[test]
fn method_malformed() {
    assert_eq!(400, util::request("/", "G", "HTTP/1.1", vec!["Host: Yeet".to_string()], ""));
}

#[test]
fn method_unknown() {
    assert_eq!(400, util::request("/", "HELLO", "HTTP/1.1", vec!["Host: Yeet".to_string()], ""));
}

#[test]
fn url_missing() {
    assert_eq!(400, util::request("", "GET", "HTTP/1.1", vec!["Host: Yeet".to_string()], ""));
}

#[test]
fn protocol_missing() {
    assert_eq!(400, util::request("/", "GET", "", vec!["Host: Yeet".to_string()], ""));
}

#[test]
fn protocol_malformed() {
    assert_eq!(400, util::request("/", "GET", "HTTP", vec!["Host: Yeet".to_string()], ""));
}

#[test]
fn protocol_invalid() {
    assert_eq!(400, util::request("/", "GET", "HTTP/9.9", vec!["Host: Yeet".to_string()], ""));
}

#[test]
fn headers_missing() {
    assert_eq!(400, util::request("/", "GET", "HTTP/1.1", vec![], ""));
}

#[test]
fn headers_malformed() {
    assert_eq!(400, util::request("/", "GET", "HTTP/1.1", vec!["Host; Yeet".to_string()], ""));
}

#[test]
fn content_length_negative() {
    assert_eq!(400, util::request("/", "GET", "HTTP/1.1", vec!["Content-Length: -10".to_string()], ""));
}

#[test]
fn content_length_non_numeric() {
    assert_eq!(400, util::request("/", "GET", "HTTP/1.1", vec!["Content-Length: abc".to_string()], ""));
}

#[test]
fn content_length_conflict() {
    assert_eq!(400, util::request("/", "GET", "HTTP/1.1", vec!["Content-Length: 19".to_string()], "Hello"));
}

#[test]
fn chunked() {
    assert_eq!(200, util::request("/", "GET", "HTTP/1.1", vec!["Transfer-encoding: chunked".to_string()], "4\r\nWiki\r\n7\r\npedia i\r\nB\r\nn \r\nchunks.\r\n0\r\n\r\n"));
}

#[test]
fn volume() {
    for _ in 0..1000 {
        util::request("/", "GET", "HTTP/1.1", vec!["Transfer-encoding: chunked".to_string()], "4\r\nWiki\r\n7\r\npedia i\r\nB\r\nn \r\nchunks.\r\n0\r\n\r\n");
        sleep(Duration::from_millis(5))
    }

    assert_eq!(200, util::request("/", "GET", "HTTP/1.1", vec!["Transfer-encoding: chunked".to_string()], "4\r\nWiki\r\n7\r\npedia i\r\nB\r\nn \r\nchunks.\r\n0\r\n\r\n"));
}
