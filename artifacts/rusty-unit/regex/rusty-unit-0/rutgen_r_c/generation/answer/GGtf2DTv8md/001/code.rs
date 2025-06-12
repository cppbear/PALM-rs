// Answer 0

#[test]
fn test_as_bytes_non_empty() {
    let input = CharInput(b"hello");
    assert_eq!(input.as_bytes(), b"hello");
}

#[test]
fn test_as_bytes_empty() {
    let input = CharInput(b"");
    assert_eq!(input.as_bytes(), b"");
}

#[test]
fn test_as_bytes_with_unicode() {
    let input = CharInput(b"hello \xe2\x9c\x94"); // "hello ✔"
    assert_eq!(input.as_bytes(), b"hello \xe2\x9c\x94");
}

