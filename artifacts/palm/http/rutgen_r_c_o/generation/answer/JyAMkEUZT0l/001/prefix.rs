// Answer 0

#[test]
fn test_from_str_valid_ascii() {
    let _ = HeaderValue::from_str(" ");
    let _ = HeaderValue::from_str("~");
    let _ = HeaderValue::from_str("!!!");
    let _ = HeaderValue::from_str("123");
    let _ = HeaderValue::from_str("hello");
    let _ = HeaderValue::from_str("header-value");
}

#[test]
fn test_from_str_invalid_control_character() {
    let _ = HeaderValue::from_str("\n");
    let _ = HeaderValue::from_str("\r");
    let _ = HeaderValue::from_str("\t");
}

#[test]
fn test_from_str_invalid_non_ascii() {
    let _ = HeaderValue::from_str("你好");
    let _ = HeaderValue::from_str("😀");
}

#[test]
fn test_from_str_empty_string() {
    let val = HeaderValue::from_str("");
}

#[test]
fn test_from_str_exceed_length() {
    let val = HeaderValue::from_str("12345678901"); // 11 characters
}

