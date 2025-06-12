// Answer 0

#[test]
fn test_from_bytes_invalid_get() {
    let input = b"GET";
    let result = Method::from_bytes(input);
    assert_eq!(result, Ok(Method(Method::Get)));
}

#[test]
fn test_from_bytes_invalid_put() {
    let input = b"PUT";
    let result = Method::from_bytes(input);
    assert_eq!(result, Ok(Method(Method::Put)));
}

#[test]
fn test_from_bytes_extension_inline_invalid() {
    let input = b"XYZ";
    let result = Method::from_bytes(input);
    assert!(result.is_ok());
}

#[test]
fn test_from_bytes_empty() {
    let input: &[u8] = &[];
    let result = Method::from_bytes(input);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_invalid_length() {
    let input = b"abcd"; // Invalid, length does not match test case
    let result = Method::from_bytes(input);
    assert!(result.is_ok());
}

