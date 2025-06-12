// Answer 0

#[test]
fn test_as_bytes_non_empty() {
    let input = ByteInput { text: b"hello", only_utf8: true };
    let bytes = input.as_bytes();
    assert_eq!(bytes, b"hello");
}

#[test]
fn test_as_bytes_empty() {
    let input = ByteInput { text: &[], only_utf8: true };
    let bytes = input.as_bytes();
    assert_eq!(bytes, &[]);
}

#[test]
fn test_as_bytes_non_utf8() {
    let input = ByteInput { text: b"\xFF\xFE\xFD", only_utf8: false };
    let bytes = input.as_bytes();
    assert_eq!(bytes, b"\xFF\xFE\xFD");
}

