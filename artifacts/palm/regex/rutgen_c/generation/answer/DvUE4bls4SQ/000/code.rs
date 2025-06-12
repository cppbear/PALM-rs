// Answer 0

#[test]
fn test_len_non_empty() {
    let input = ByteInput { text: b"hello", only_utf8: true };
    assert_eq!(input.len(), 5);
}

#[test]
fn test_len_empty() {
    let input = ByteInput { text: b"", only_utf8: true };
    assert_eq!(input.len(), 0);
}

#[test]
fn test_len_bytes() {
    let input = ByteInput { text: b"1234567890", only_utf8: true };
    assert_eq!(input.len(), 10);
}

