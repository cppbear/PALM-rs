// Answer 0

#[test]
fn test_from_str_valid() {
    let val = HeaderValue::from_str("hello").unwrap();
    assert_eq!(val.as_bytes(), b"hello");
}

#[test]
fn test_from_str_invalid() {
    let val = HeaderValue::from_str("\n");
    assert!(val.is_err());
}

#[test]
fn test_from_str_empty() {
    let val = HeaderValue::from_str("");
    assert!(val.is_ok());
    assert!(val.unwrap().is_empty());
}

#[test]
fn test_from_str_control_character() {
    let val = HeaderValue::from_str("\x00");
    assert!(val.is_err());
}

#[test]
fn test_from_str_max_length() {
    let input = "a".repeat(127); // 127 is the maximum valid length for ASCII characters
    let val = HeaderValue::from_str(&input);
    assert!(val.is_ok());
    assert_eq!(val.unwrap().len(), input.len());
}

