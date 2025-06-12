// Answer 0

#[test]
fn test_from_utf8_lossy_valid_utf8() {
    let bytes = b"Hello, world!";
    let result = from_utf8_lossy(bytes);
    assert_eq!(result, "Hello, world!");
}

#[test]
fn test_from_utf8_lossy_invalid_utf8_single() {
    let bytes = &[0xFF]; // Invalid UTF-8 byte
    let result = from_utf8_lossy(bytes);
    assert_eq!(result, "\u{fffd}"); // Replacement character
}

#[test]
fn test_from_utf8_lossy_invalid_utf8_multiple() {
    let bytes = &[0xFF, 0xFF, 0xFF]; // Invalid UTF-8 bytes
    let result = from_utf8_lossy(bytes);
    assert_eq!(result, "\u{fffd}\u{fffd}\u{fffd}"); // Replacement characters
}

#[test]
fn test_from_utf8_lossy_mixed_valid_invalid() {
    let bytes = b"Hello, \xFFworld!";
    let result = from_utf8_lossy(bytes);
    assert_eq!(result, "Hello, \u{fffd}world!"); // Replacement character for invalid byte
}

#[test]
fn test_from_utf8_lossy_empty() {
    let bytes = b""; // Empty byte slice
    let result = from_utf8_lossy(bytes);
    assert_eq!(result, ""); // Empty string
}

