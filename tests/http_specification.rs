mod common;

#[test]
fn basic() {
    let result = common::request("http://localhost".to_string(), "GET".to_string());

    assert_eq!(result.url, "http://localhost".to_string());
}

