// Answer 0

#[test]
fn test_escape_byte_zero() {
    let result = escape_byte(0);
    assert_eq!(result, "\\0");
}

#[test]
fn test_escape_byte_backspace() {
    let result = escape_byte(8);
    assert_eq!(result, "\\x08");
}

#[test]
fn test_escape_byte_tab() {
    let result = escape_byte(9);
    assert_eq!(result, "\\t");
}

#[test]
fn test_escape_byte_line_feed() {
    let result = escape_byte(10);
    assert_eq!(result, "\\n");
}

#[test]
fn test_escape_byte_carriage_return() {
    let result = escape_byte(13);
    assert_eq!(result, "\\r");
}

#[test]
fn test_escape_byte_special_characters() {
    let result = escape_byte(27);
    assert_eq!(result, "\\x1b");
}

#[test]
fn test_escape_byte_ascii() {
    let result = escape_byte(65);
    assert_eq!(result, "A");
}

#[test]
fn test_escape_byte_max() {
    let result = escape_byte(255);
    assert_eq!(result, "\\xff");
}

