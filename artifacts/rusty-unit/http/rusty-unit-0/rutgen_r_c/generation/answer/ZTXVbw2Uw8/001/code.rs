// Answer 0

#[test]
fn test_as_bytes_non_empty() {
    let header_value = HeaderValue::from_static("test");
    let bytes = header_value.as_bytes();
    assert_eq!(bytes, b"test");
}

#[test]
fn test_as_bytes_empty() {
    let header_value = HeaderValue::from_static("");
    let bytes = header_value.as_bytes();
    assert_eq!(bytes, b"");
}

#[test]
fn test_as_bytes_with_special_characters() {
    let header_value = HeaderValue::from_static("value_with_special_#@!");
    let bytes = header_value.as_bytes();
    assert_eq!(bytes, b"value_with_special_#@!");
}

#[test]
fn test_as_bytes_large_value() {
    let large_string = "a".repeat(1000);
    let header_value = HeaderValue::from_static(&large_string);
    let bytes = header_value.as_bytes();
    assert_eq!(bytes, large_string.as_bytes());
}

