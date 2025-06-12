// Answer 0

#[test]
fn test_len_non_empty() {
    let val = HeaderValue::from_static("hello");
    assert_eq!(val.len(), 5);
}

#[test]
fn test_len_empty() {
    let val = HeaderValue::from_static("");
    assert_eq!(val.len(), 0);
}

#[test]
fn test_len_with_non_ascii() {
    let val = HeaderValue::from_static("こんにちは");
    assert_eq!(val.len(), 15); // Length of "こんにちは" in bytes
}

#[test]
fn test_len_with_spaces() {
    let val = HeaderValue::from_static("   ");
    assert_eq!(val.len(), 3);
}

