// Answer 0

#[test]
fn test_valid_header_value() {
    let val = http::header::HeaderValue::from_static("hello");
    assert_eq!(val.inner, http::Bytes::from_static(b"hello"));
}

#[test]
#[should_panic(expected = "index out of bounds: the length is 0 but the index is 0")]
fn test_invalid_header_value_contains_non_visible_character() {
    let _ = http::header::HeaderValue::from_static("hello\x00world");
}

#[test]
#[should_panic(expected = "index out of bounds: the length is 0 but the index is 0")]
fn test_invalid_header_value_contains_non_ascii_character() {
    let _ = http::header::HeaderValue::from_static("helloжworld");
}

#[test]
fn test_empty_header_value() {
    let val = http::header::HeaderValue::from_static("");
    assert_eq!(val.inner, http::Bytes::from_static(b""));
}

#[test]
fn test_boundary_visible_character() {
    let val = http::header::HeaderValue::from_static(" ");
    assert_eq!(val.inner, http::Bytes::from_static(b" "));
    
    let val = http::header::HeaderValue::from_static("~");
    assert_eq!(val.inner, http::Bytes::from_static(b"~"));
}

