// Answer 0

#[test]
fn test_header_value_is_empty_with_empty_string() {
    let val = HeaderValue::from_static("");
    assert!(val.is_empty());
}

#[test]
fn test_header_value_is_empty_with_non_empty_string() {
    let val = HeaderValue::from_static("hello");
    assert!(!val.is_empty());
}

#[test]
fn test_header_value_is_empty_with_bytes() {
    let val = HeaderValue::from_bytes(b"").unwrap();
    assert!(val.is_empty());
}

#[test]
fn test_header_value_is_empty_with_non_empty_bytes() {
    let val = HeaderValue::from_bytes(b"hello").unwrap();
    assert!(!val.is_empty());
}

