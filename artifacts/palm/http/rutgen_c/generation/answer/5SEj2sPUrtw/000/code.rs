// Answer 0

#[test]
#[should_panic]
fn test_from_static_invalid_character() {
    const _: HeaderValue = HeaderValue::from_static("жsome value"); 
}

#[test]
fn test_from_static_valid_string() {
    let val = HeaderValue::from_static("hello");
    assert_eq!(val.as_bytes(), b"hello");
}

#[test]
fn test_from_static_empty_string() {
    const _: HeaderValue = HeaderValue::from_static(""); // Should not panic, as empty string is valid
}

