// Answer 0

#[test]
fn test_error_code_display_message() {
    let msg = Box::from("test message");
    let error_code = ErrorCode::Message(msg);
    let mut output = String::new();
    assert_eq!(write!(&mut output, "{}", error_code).is_ok(), true);
    assert_eq!(output, "test message");
}

#[test]
fn test_error_code_io() {
    use std::io;
    let io_error = io::Error::new(io::ErrorKind::Other, "I/O error occurred");
    let error_code = ErrorCode::Io(io_error);
    let mut output = String::new();
    assert_eq!(write!(&mut output, "{}", error_code).is_ok(), true);
    assert_eq!(output, "I/O error occurred");
}

#[test]
fn test_error_code_eof_while_parsing_list() {
    let error_code = ErrorCode::EofWhileParsingList;
    let mut output = String::new();
    assert_eq!(write!(&mut output, "{}", error_code).is_ok(), true);
    assert_eq!(output, "EOF while parsing a list");
}

#[test]
fn test_error_code_eof_while_parsing_object() {
    let error_code = ErrorCode::EofWhileParsingObject;
    let mut output = String::new();
    assert_eq!(write!(&mut output, "{}", error_code).is_ok(), true);
    assert_eq!(output, "EOF while parsing an object");
}

#[test]
fn test_error_code_eof_while_parsing_string() {
    let error_code = ErrorCode::EofWhileParsingString;
    let mut output = String::new();
    assert_eq!(write!(&mut output, "{}", error_code).is_ok(), true);
    assert_eq!(output, "EOF while parsing a string");
}

#[test]
fn test_error_code_eof_while_parsing_value() {
    let error_code = ErrorCode::EofWhileParsingValue;
    let mut output = String::new();
    assert_eq!(write!(&mut output, "{}", error_code).is_ok(), true);
    assert_eq!(output, "EOF while parsing a value");
}

#[test]
fn test_error_code_expected_colon() {
    let error_code = ErrorCode::ExpectedColon;
    let mut output = String::new();
    assert_eq!(write!(&mut output, "{}", error_code).is_ok(), true);
    assert_eq!(output, "expected `:`");
}

#[test]
fn test_error_code_unexpected_end_of_hex_escape() {
    let error_code = ErrorCode::UnexpectedEndOfHexEscape;
    let mut output = String::new();
    assert_eq!(write!(&mut output, "{}", error_code).is_ok(), true);
    assert_eq!(output, "unexpected end of hex escape");
}

#[test]
fn test_error_code_recursion_limit_exceeded() {
    let error_code = ErrorCode::RecursionLimitExceeded;
    let mut output = String::new();
    assert_eq!(write!(&mut output, "{}", error_code).is_ok(), true);
    assert_eq!(output, "recursion limit exceeded");
}

