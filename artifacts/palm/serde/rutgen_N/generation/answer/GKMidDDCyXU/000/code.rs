// Answer 0

#[test]
fn test_from_utf8_lossy_valid_utf8() {
    let valid_bytes = b"Hello, world!";
    let result = serde::from_utf8_lossy(valid_bytes);
    assert_eq!(result, "Hello, world!");
}

#[test]
fn test_from_utf8_lossy_invalid_utf8() {
    let invalid_bytes = [0xFF, 0xFE, 0xFD]; // Invalid UTF-8 bytes
    let result = serde::from_utf8_lossy(&invalid_bytes);
    assert_eq!(result, "\u{FFFD}\u{FFFD}\u{FFFD}");
}

#[test]
fn test_from_utf8_lossy_empty() {
    let empty_bytes: &[u8] = &[];
    let result = serde::from_utf8_lossy(empty_bytes);
    assert_eq!(result, "");
}

#[test]
fn test_from_utf8_lossy_boundary() {
    let boundary_bytes = b"Test \xC2\xA9"; // Valid UTF-8 followed by a valid multi-byte character
    let result = serde::from_utf8_lossy(boundary_bytes);
    assert_eq!(result, "Test ©");
}

