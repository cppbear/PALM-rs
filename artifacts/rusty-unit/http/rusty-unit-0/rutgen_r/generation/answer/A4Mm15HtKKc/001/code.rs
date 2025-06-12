// Answer 0

#[test]
fn test_len_static_string() {
    let val = HeaderValue::from_static("hello");
    assert_eq!(val.len(), 5);
}

#[test]
fn test_len_empty_string() {
    let val = HeaderValue::from_static("");
    assert_eq!(val.len(), 0);
}

#[test]
fn test_len_long_string() {
    let val = HeaderValue::from_static("a long header value");
    assert_eq!(val.len(), 20);
}

#[test]
fn test_len_special_characters() {
    let val = HeaderValue::from_static("!@#$%^&*()");
    assert_eq!(val.len(), 10);
}

#[test]
fn test_len_unicode_characters() {
    let val = HeaderValue::from_static("こんにちは");
    assert_eq!(val.len(), 15); // 5 characters, each represented by 3 bytes
}

#[test]
fn test_len_single_character() {
    let val = HeaderValue::from_static("A");
    assert_eq!(val.len(), 1);
}

