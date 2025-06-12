// Answer 0

#[test]
fn test_vb_non_ascii() {
    let input = 256; // A byte value greater than u8::MAX
    let result = vb(input);
    assert_eq!(result, "EOF");
}

#[test]
fn test_vb_ascii() {
    let input = 65; // ASCII value for 'A'
    let result = vb(input);
    assert_eq!(result, "A");
}

#[test]
fn test_vb_escape() {
    let input = 9; // ASCII value for TAB
    let result = vb(input);
    assert_eq!(result, "\\t");
}

#[test]
fn test_vb_control_character() {
    let input = 10; // ASCII value for LF (Line Feed)
    let result = vb(input);
    assert_eq!(result, "\\n");
}

#[test]
fn test_vb_multiple_byte_value() {
    let input = 255; // Maximum value for u8
    let result = vb(input);
    assert_eq!(result, "\\xFF");
}

