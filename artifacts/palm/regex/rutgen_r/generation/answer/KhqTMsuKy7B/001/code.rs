// Answer 0

#[test]
fn test_escape_byte_ascii() {
    let input: u8 = 65; // 'A'
    let expected_output = "\\x41";

    assert_eq!(escape_byte(input), expected_output);
}

#[test]
fn test_escape_byte_non_ascii() {
    let input: u8 = 200; // non-ASCII byte
    let expected_output = "\\xC8";

    assert_eq!(escape_byte(input), expected_output);
}

#[test]
fn test_escape_byte_control_character() {
    let input: u8 = 7; // Bell character
    let expected_output = "\\x07";

    assert_eq!(escape_byte(input), expected_output);
}

#[test]
fn test_escape_byte_zero() {
    let input: u8 = 0; // Null character
    let expected_output = "\\x00";

    assert_eq!(escape_byte(input), expected_output);
}

#[test]
fn test_escape_byte_boundary() {
    let input: u8 = 255; // highest value for u8
    let expected_output = "\\xFF";

    assert_eq!(escape_byte(input), expected_output);
}

