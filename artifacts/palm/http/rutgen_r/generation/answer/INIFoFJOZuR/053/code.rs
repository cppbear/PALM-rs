// Answer 0

#[test]
fn test_from_bytes_get() {
    let result = http::from_bytes(b"GET");
    assert_eq!(result, Ok(http::Method::Get));
}

#[test]
fn test_from_bytes_put() {
    let result = http::from_bytes(b"PUT");
    assert_eq!(result, Ok(http::Method::Put));
}

#[test]
fn test_from_bytes_invalid_method_length() {
    let result = http::from_bytes(b"");
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_invalid_method_extension() {
    let result = http::from_bytes(b"XYZ");
    assert!(result.is_err());
} 

#[test]
fn test_from_bytes_valid_extension_method() {
    let result = http::from_bytes(b"EXTRA");
    assert!(result.is_ok());
}

