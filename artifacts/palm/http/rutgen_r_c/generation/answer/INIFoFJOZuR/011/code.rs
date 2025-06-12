// Answer 0

#[test]
fn test_from_bytes_options() {
    let result = Method::from_bytes(b"OPTIONS");
    assert_eq!(result, Ok(Method(Method::OPTIONS)));
}

#[test]
fn test_from_bytes_connect() {
    let result = Method::from_bytes(b"CONNECT");
    assert_eq!(result, Ok(Method(Method::CONNECT)));
}

#[test]
fn test_from_bytes_empty() {
    let result = Method::from_bytes(b"");
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_invalid_length() {
    let result = Method::from_bytes(b"INVALID");
    assert!(result.is_err());
}

