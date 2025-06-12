// Answer 0

#[test]
fn test_error_code_message() {
    let error_code = ErrorCode::Message(Box::from("Test error message"));
    let mut output = String::new();
    let result = error_code.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "Test error message");
}

#[test]
fn test_error_code_io() {
    use std::io;

    let io_error = io::Error::new(io::ErrorKind::Other, "I/O error occurred");
    let error_code = ErrorCode::Io(io_error);
    let mut output = String::new();
    let result = error_code.fmt(&mut output);
    assert!(result.is_ok());
    assert!(output.contains("I/O error occurred"));
}

#[test]
fn test_error_code_eof_while_parsing_list() {
    let error_code = ErrorCode::EofWhileParsingList;
    let mut output = String::new();
    let result = error_code.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "EOF while parsing a list");
}

#[test]
fn test_error_code_eof_while_parsing_object() {
    let error_code = ErrorCode::EofWhileParsingObject;
    let mut output = String::new();
    let result = error_code.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "EOF while parsing an object");
}

#[test]
fn test_error_code_expected_colon() {
    let error_code = ErrorCode::ExpectedColon;
    let mut output = String::new();
    let result = error_code.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "expected `:`");
}

#[test]
fn test_error_code_trailing_characters() {
    let error_code = ErrorCode::TrailingCharacters;
    let mut output = String::new();
    let result = error_code.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "trailing characters");
}

#[test]
fn test_error_code_recursion_limit_exceeded() {
    let error_code = ErrorCode::RecursionLimitExceeded;
    let mut output = String::new();
    let result = error_code.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "recursion limit exceeded");
}

