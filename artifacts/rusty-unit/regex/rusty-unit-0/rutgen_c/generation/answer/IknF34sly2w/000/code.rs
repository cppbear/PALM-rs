// Answer 0

#[test]
fn test_escape_bytes_empty() {
    let result = escape_bytes(&[]);
    assert_eq!(result, "");
}

#[test]
fn test_escape_bytes_single_byte() {
    let result = escape_bytes(&[10]); // newline character
    assert_eq!(result, "\\n");
}

#[test]
fn test_escape_bytes_multiple_bytes() {
    let result = escape_bytes(&[32, 33, 34]); // space, exclamation mark, double quote
    assert_eq!(result, " \\!\"");
}

#[test]
fn test_escape_bytes_bytes_with_non_ascii() {
    let result = escape_bytes(&[255]); // maximum byte value
    assert_eq!(result, "\\xFF");
}

#[test]
fn test_escape_bytes_control_characters() {
    let result = escape_bytes(&[0, 1, 2]); // null, start of header, start of text
    assert_eq!(result, "\\x00\\x01\\x02");
}

