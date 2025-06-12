// Answer 0

#[test]
fn test_error_code_message() {
    let msg = Box::from("An error occurred");
    let error_code = ErrorCode::Message(msg);
    
    let mut output = String::new();
    let result = write!(&mut output, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(output, "An error occurred");
}

#[test]
fn test_error_code_io() {
    let io_error = std::io::Error::new(std::io::ErrorKind::Other, "IO error");
    let error_code = ErrorCode::Io(io_error);
    
    let mut output = String::new();
    let result = write!(&mut output, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(output, "IO error");
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
fn test_error_code_invalid_unicode_code_point() {
    let error_code = ErrorCode::InvalidUnicodeCodePoint;
    
    let mut output = String::new();
    let result = write!(&mut output, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(output, "invalid unicode code point");
}

#[test]
fn test_error_code_expected_colon() {
    let error_code = ErrorCode::ExpectedColon;
    
    let mut output = String::new();
    let result = write!(&mut output, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(output, "expected `:`");
}

#[test]
fn test_error_code_trailing_characters() {
    let error_code = ErrorCode::TrailingCharacters;
    
    let mut output = String::new();
    let result = write!(&mut output, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(output, "trailing characters");
}

