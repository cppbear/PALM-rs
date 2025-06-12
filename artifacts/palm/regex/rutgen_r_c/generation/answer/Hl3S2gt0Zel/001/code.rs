// Answer 0

#[test]
fn test_escape_byte_valid_ascii() {
    let input: u8 = b'a'; // ASCII 'a'
    let expected = String::from("\\x61");
    let result = escape_byte(input);
    assert_eq!(result, expected);
}

#[test]
fn test_escape_byte_valid_special_character() {
    let input: u8 = b'\n'; // Newline character
    let expected = String::from("\\n");
    let result = escape_byte(input);
    assert_eq!(result, expected);
}

#[test]
fn test_escape_byte_valid_non_ascii() {
    let input: u8 = 0x7F; // DEL character
    let expected = String::from("\\x7f");
    let result = escape_byte(input);
    assert_eq!(result, expected);
}

#[test]
fn test_escape_byte_valid_control_character() {
    let input: u8 = 0x01; // Start of Heading
    let expected = String::from("\\x01");
    let result = escape_byte(input);
    assert_eq!(result, expected);
}

#[test]
fn test_escape_byte_upper_boundary() {
    let input: u8 = 255; // Highest value for u8
    let expected = String::from("\\xff");
    let result = escape_byte(input);
    assert_eq!(result, expected);
}

