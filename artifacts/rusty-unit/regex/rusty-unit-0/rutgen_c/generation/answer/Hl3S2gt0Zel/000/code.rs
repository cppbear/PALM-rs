// Answer 0

#[test]
fn test_escape_byte_ascii() {
    assert_eq!(escape_byte(0x41), "A"); // ASCII 'A'
    assert_eq!(escape_byte(0x61), "a"); // ASCII 'a'
    assert_eq!(escape_byte(0x30), "0"); // ASCII '0'
}

#[test]
fn test_escape_byte_control() {
    assert_eq!(escape_byte(0x00), "\\0"); // Null byte
    assert_eq!(escape_byte(0x07), "\\a"); // Bell
    assert_eq!(escape_byte(0x08), "\\b"); // Backspace
    assert_eq!(escape_byte(0x0C), "\\f"); // Formfeed
    assert_eq!(escape_byte(0x0A), "\\n"); // Newline
    assert_eq!(escape_byte(0x0D), "\\r"); // Carriage return
    assert_eq!(escape_byte(0x1B), "\\e"); // Escape
}

#[test]
fn test_escape_byte_special() {
    assert_eq!(escape_byte(0xFF), "\\xFF"); // Extended byte
}

