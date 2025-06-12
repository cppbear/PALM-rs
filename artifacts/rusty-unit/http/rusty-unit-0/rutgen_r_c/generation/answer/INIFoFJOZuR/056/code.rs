// Answer 0

#[test]
fn test_from_bytes_get() {
    let input = b"GET";
    let result = Method::from_bytes(input);
    assert_eq!(result, Ok(Method(Method::GET.0.clone())));
}

#[test]
fn test_from_bytes_put() {
    let input = b"PUT";
    let result = Method::from_bytes(input);
    assert_eq!(result, Ok(Method(Method::PUT.0.clone())));
}

#[test]
fn test_from_bytes_invalid_empty() {
    let input: &[u8] = &[];
    let result = Method::from_bytes(input);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_invalid_unknown() {
    let input = b"XYZ";
    let result = Method::from_bytes(input);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_extensions() {
    let input = b"EXTENSION";
    let result = Method::from_bytes(input);
    assert!(result.is_ok());
}

