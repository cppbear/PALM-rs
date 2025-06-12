// Answer 0

#[test]
fn test_error_code_message() {
    let message = Box::from("An error occurred");
    let error_code = ErrorCode::Message(message);
    let mut output = String::new();
    let result = write!(&mut output, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(output, "An error occurred");
}

#[test]
fn test_error_code_io() {
    use std::io::{self, ErrorKind};
    
    let io_error = io::Error::new(ErrorKind::Other, "I/O error occurred");
    let error_code = ErrorCode::Io(io_error);
    let mut output = String::new();
    
    let result = write!(&mut output, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(output, "I/O error occurred");
}

#[test]
fn test_error_code_eof_while_parsing_list() {
    let error_code = ErrorCode::EofWhileParsingList;
    let mut output = String::new();
    
    let result = write!(&mut output, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(output, "EOF while parsing a list");
}

#[test]
fn test_error_code_eof_while_parsing_object() {
    let error_code = ErrorCode::EofWhileParsingObject;
    let mut output = String::new();
    
    let result = write!(&mut output, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(output, "EOF while parsing an object");
}

#[test]
fn test_error_code_invalid_number() {
    let error_code = ErrorCode::InvalidNumber;
    let mut output = String::new();
    
    let result = write!(&mut output, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(output, "invalid number");
}

#[test]
fn test_error_code_trailing_comma() {
    let error_code = ErrorCode::TrailingComma;
    let mut output = String::new();
    
    let result = write!(&mut output, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(output, "trailing comma");
}

