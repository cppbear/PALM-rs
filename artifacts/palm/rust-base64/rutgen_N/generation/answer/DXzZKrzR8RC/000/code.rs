// Answer 0

#[test]
fn test_encode_empty() {
    let result = base64::encode(b"");
    assert_eq!(result, "");
}

#[test]
fn test_encode_single_byte() {
    let result = base64::encode(b"a");
    assert_eq!(result, "YQ==");
}

#[test]
fn test_encode_two_bytes() {
    let result = base64::encode(b"ab");
    assert_eq!(result, "YWJ=");
}

#[test]
fn test_encode_three_bytes() {
    let result = base64::encode(b"abc");
    assert_eq!(result, "YWJj");
}

#[test]
fn test_encode_four_bytes() {
    let result = base64::encode(b"abcd");
    assert_eq!(result, "YWJjZA==");
}

#[test]
fn test_encode_non_ascii() {
    let result = base64::encode(b"\xF0\x9F\x98\x81"); // U+1F601 (😀)
    assert_eq!(result, "8J+YgA==");
}

