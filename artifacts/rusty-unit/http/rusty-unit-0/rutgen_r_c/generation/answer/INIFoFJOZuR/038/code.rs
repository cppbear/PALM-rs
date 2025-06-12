// Answer 0

#[test]
fn test_from_bytes_patch() {
    let input = b"PATCH";
    let result = Method::from_bytes(input);
    assert_eq!(result, Ok(Method(Method::Patch)));
}

#[test]
fn test_from_bytes_invalid_empty() {
    let input = b"";
    let result = Method::from_bytes(input);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_invalid_unknown() {
    let input = b"UNKNOWN";
    let result = Method::from_bytes(input);
    assert!(result.is_ok()); // Should be treated as extension
}

#[test]
fn test_from_bytes_invalid_too_long() {
    let input = b"TOOLONGMETHOD";
    let result = Method::from_bytes(input);
    assert!(result.is_ok()); // Should be treated as extension
}

