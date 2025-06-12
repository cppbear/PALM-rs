// Answer 0

#[test]
fn test_as_bytes_empty() {
    let value = HeaderValue::from_static("");
    let result = value.as_bytes();
}

#[test]
fn test_as_bytes_1_char() {
    let value = HeaderValue::from_static("a");
    let result = value.as_bytes();
}

#[test]
fn test_as_bytes_10_chars() {
    let value = HeaderValue::from_static("abcdefghij");
    let result = value.as_bytes();
}

#[test]
fn test_as_bytes_20_chars() {
    let value = HeaderValue::from_static("abcdefghijabcdefghij");
    let result = value.as_bytes();
}

#[test]
#[should_panic]
fn test_as_bytes_over_20_chars() {
    let value = HeaderValue::from_static("abcdefghijabcdefghijk"); // 21 chars
    let result = value.as_bytes();
}

