// Answer 0

#[test]
fn test_from_bytes_get() {
    let result = Method::from_bytes(b"GET");
    assert_eq!(result, Ok(Method(Method::GET.0.clone())));
}

#[test]
fn test_from_bytes_put() {
    let result = Method::from_bytes(b"PUT");
    assert_eq!(result, Ok(Method(Method::PUT.0.clone())));
}

#[test]
#[should_panic]
fn test_from_bytes_empty() {
    // This test should panic or fail when input is empty
    let _ = Method::from_bytes(b"");
}

#[test]
fn test_from_bytes_invalid_method() {
    let result = Method::from_bytes(b"INVALID");
    assert!(result.is_err());
}

