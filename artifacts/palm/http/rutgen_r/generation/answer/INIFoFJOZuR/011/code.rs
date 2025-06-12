// Answer 0

#[test]
fn test_from_bytes_options() {
    let result = http::from_bytes(b"OPTIONS");
    assert_eq!(result, Ok(http::Method::Options));
}

#[test]
fn test_from_bytes_connect() {
    let result = http::from_bytes(b"CONNECT");
    assert_eq!(result, Ok(http::Method::Connect));
}

