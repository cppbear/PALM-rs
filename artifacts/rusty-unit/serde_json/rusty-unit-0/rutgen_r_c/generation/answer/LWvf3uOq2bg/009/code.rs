// Answer 0

#[test]
fn test_error_code_message() {
    let message = Box::from("A sample error message");
    let error_code = ErrorCode::Message(message);
    let mut output = String::new();
    let result = write!(output, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(output, "A sample error message");
}

#[test]
fn test_error_code_eof_while_parsing_list() {
    let error_code = ErrorCode::EofWhileParsingList;
    let mut output = String::new();
    let result = write!(output, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(output, "EOF while parsing a list");
}

#[test]
fn test_error_code_eof_while_parsing_object() {
    let error_code = ErrorCode::EofWhileParsingObject;
    let mut output = String::new();
    let result = write!(output, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(output, "EOF while parsing an object");
}

#[test]
fn test_error_code_eof_while_parsing_string() {
    let error_code = ErrorCode::EofWhileParsingString;
    let mut output = String::new();
    let result = write!(output, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(output, "EOF while parsing a string");
}

#[test]
fn test_error_code_invalid_unicode_code_point() {
    let error_code = ErrorCode::InvalidUnicodeCodePoint;
    let mut output = String::new();
    let result = write!(output, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(output, "invalid unicode code point");
}

#[test]
fn test_error_code_control_character_while_parsing_string() {
    let error_code = ErrorCode::ControlCharacterWhileParsingString;
    let mut output = String::new();
    let result = write!(output, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(output, "control character (\\u0000-\\u001F) found while parsing a string");
}

