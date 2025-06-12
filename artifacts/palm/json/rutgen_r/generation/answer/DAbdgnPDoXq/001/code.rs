// Answer 0

#[test]
fn test_serialize_char() {
    let value: char = 'a';
    let result = serialize_char(value);
    assert_eq!(result, Ok("a".to_string()));
}

#[test]
fn test_serialize_char_unicode() {
    let value: char = '你'; // Chinese character
    let result = serialize_char(value);
    assert_eq!(result, Ok("你".to_string()));
}

#[test]
fn test_serialize_char_special() {
    let value: char = '!'; // Special character
    let result = serialize_char(value);
    assert_eq!(result, Ok("!".to_string()));
}

#[test]
fn test_serialize_char_numeric() {
    let value: char = '5'; // Numeric character
    let result = serialize_char(value);
    assert_eq!(result, Ok("5".to_string()));
}

#[test]
fn test_serialize_char_space() {
    let value: char = ' '; // Space character
    let result = serialize_char(value);
    assert_eq!(result, Ok(" ".to_string()));
}

#[test]
fn test_serialize_char_last_valid() {
    let value: char = '\u{10FFFF}'; // Last valid Unicode character
    let result = serialize_char(value);
    assert_eq!(result, Ok("\u{10FFFF}".to_string()));
}

