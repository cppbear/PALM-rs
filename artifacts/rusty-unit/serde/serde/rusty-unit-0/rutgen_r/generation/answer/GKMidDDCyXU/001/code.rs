// Answer 0

#[test]
fn test_from_utf8_lossy_valid_utf8() {
    let input = b"Hello, world!";
    let result = serde::from_utf8_lossy(input);
    assert_eq!(result, "Hello, world!");
}

#[test]
fn test_from_utf8_lossy_invalid_utf8() {
    let input = &[0xFF, 0xFE, 0xFD]; // Invalid bytes
    let result = serde::from_utf8_lossy(input);
    assert_eq!(result, "\u{FFFD}\u{FFFD}\u{FFFD}"); // Expected replacement characters
}

#[test]
fn test_from_utf8_lossy_empty() {
    let input: &[u8] = &[];
    let result = serde::from_utf8_lossy(input);
    assert_eq!(result, ""); // An empty byte slice should return an empty string
}

#[test]
fn test_from_utf8_lossy_mixed() {
    let input = b"Valid \xFF bytes"; // Mixed valid and invalid
    let result = serde::from_utf8_lossy(input);
    assert_eq!(result, "Valid \u{FFFD} bytes"); // Expected replacement for invalid byte
}

