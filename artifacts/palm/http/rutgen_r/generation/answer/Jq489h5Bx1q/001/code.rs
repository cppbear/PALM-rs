// Answer 0

#[test]
fn test_from_bytes_valid() {
    use http::header::HeaderValue;

    let valid_bytes = b"hello\xfa";
    let result = HeaderValue::from_bytes(valid_bytes);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), &valid_bytes[..]);
}

#[test]
fn test_from_bytes_valid_boundary() {
    use http::header::HeaderValue;

    let max_valid_bytes = b"\x20\x7f\xff";
    let result = HeaderValue::from_bytes(max_valid_bytes);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), &max_valid_bytes[..]);
}

#[test]
fn test_from_bytes_invalid_low() {
    use http::header::HeaderValue;

    let invalid_bytes = b"\n";
    let result = HeaderValue::from_bytes(invalid_bytes);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_invalid_high() {
    use http::header::HeaderValue;

    let invalid_bytes = b"\x80";
    let result = HeaderValue::from_bytes(invalid_bytes);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_invalid_empty() {
    use http::header::HeaderValue;

    let empty_bytes: &[u8] = b"";
    let result = HeaderValue::from_bytes(empty_bytes);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), &empty_bytes[..]);
}

#[test]
fn test_from_bytes_multiple_invalid() {
    use http::header::HeaderValue;

    let mixed_bytes = b"valid\x00value"; // contains null byte
    let result = HeaderValue::from_bytes(mixed_bytes);
    assert!(result.is_err());
}

