mod setup;
mod util;

#[test]
fn correct() {
    assert_eq!(200, util::request("/", "GET", "HTTP/1.1", Vec::new(), ""));
}

#[test]
fn malformed_method() {
    assert_eq!(400, util::request("/", "G", "HTTP/1.1", Vec::new(), ""));
}
