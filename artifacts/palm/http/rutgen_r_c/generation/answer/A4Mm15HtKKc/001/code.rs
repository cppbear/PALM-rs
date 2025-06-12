// Answer 0

#[test]
fn test_len_empty() {
    let val = HeaderValue::from_static("");
    assert_eq!(val.len(), 0);
}

#[test]
fn test_len_single_character() {
    let val = HeaderValue::from_static("a");
    assert_eq!(val.len(), 1);
}

#[test]
fn test_len_multiple_characters() {
    let val = HeaderValue::from_static("hello");
    assert_eq!(val.len(), 5);
}

#[test]
fn test_len_unicode_characters() {
    let val = HeaderValue::from_static("こんにちは");
    assert_eq!(val.len(), 15); // Assuming UTF-8 encoding for each character
}

#[test]
fn test_len_large_string() {
    let val = HeaderValue::from_static("a".repeat(1000));
    assert_eq!(val.len(), 1000);
}

#[test]
fn test_len_null_characters() {
    let val = HeaderValue::from_static("hello\0world");
    assert_eq!(val.len(), 11);
}

#[test]
fn test_len_whitespace_characters() {
    let val = HeaderValue::from_static("   ");
    assert_eq!(val.len(), 3);
}

