// Answer 0

#[test]
fn test_from_bytes_valid_delete() {
    let input: &[u8] = b"DELETE";
    let result = Method::from_bytes(input);
    assert_eq!(result, Ok(Method(Method::Inner::Delete)));
}

#[test]
fn test_from_bytes_invalid_empty() {
    let input: &[u8] = b"";
    let result = Method::from_bytes(input);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_invalid_unknown_method() {
    let input: &[u8] = b"UNKNOWN";
    let result = Method::from_bytes(input);
    assert!(result.is_err());
}

