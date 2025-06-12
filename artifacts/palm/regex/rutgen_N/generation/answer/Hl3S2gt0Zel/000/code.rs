// Answer 0

#[test]
fn test_escape_byte_control_character() {
    let result = escape_byte(0); // Testing the null byte
    assert_eq!(result, "\\0");
}

#[test]
fn test_escape_byte_backspace() {
    let result = escape_byte(8); // Testing the backspace byte
    assert_eq!(result, "\\x08");
}

#[test]
fn test_escape_byte_tab() {
    let result = escape_byte(9); // Testing the tab byte
    assert_eq!(result, "\\t");
}

#[test]
fn test_escape_byte_linefeed() {
    let result = escape_byte(10); // Testing the line feed byte
    assert_eq!(result, "\\n");
}

#[test]
fn test_escape_byte_carriage_return() {
    let result = escape_byte(13); // Testing the carriage return byte
    assert_eq!(result, "\\r");
}

#[test]
fn test_escape_byte_ascii() {
    let result = escape_byte(65); // Testing the ASCII character 'A'
    assert_eq!(result, "A");
}

#[test]
fn test_escape_byte_non_ascii() {
    let result = escape_byte(255); // Testing the byte value 255
    assert_eq!(result, "\\xFF");
}

