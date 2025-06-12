// Answer 0

#[test]
fn test_as_bytes_non_empty() {
    let header_value = HeaderValue::from_static("hello");
    assert_eq!(header_value.as_bytes(), b"hello");
}

#[test]
fn test_as_bytes_empty() {
    let header_value = HeaderValue::from_static("");
    assert_eq!(header_value.as_bytes(), b"");
}

#[test]
fn test_as_bytes_sensitive() {
    let mut header_value = HeaderValue::from_static("secret");
    header_value.set_sensitive(true);
    assert_eq!(header_value.as_bytes(), b"secret");
}

