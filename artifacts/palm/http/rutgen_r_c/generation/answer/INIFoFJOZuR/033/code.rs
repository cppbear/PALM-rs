// Answer 0

#[test]
fn test_from_bytes_patch() {
    let input = b"PATCH";
    let result = Method::from_bytes(input);
    assert_eq!(result, Ok(Method(Method::Patch)));
}

#[test]
fn test_from_bytes_trace() {
    let input = b"TRACE";
    let result = Method::from_bytes(input);
    assert_eq!(result, Ok(Method(Method::Trace)));
}

