// Answer 0

#[test]
fn test_fmt_eof_while_parsing_list() {
    let error_code = ErrorCode::EofWhileParsingValue;
    let mut output = String::new();
    
    let result = std::fmt::write(&mut output, format_args!("{}", error_code));
    
    assert!(result.is_ok());
    assert_eq!(output, "EOF while parsing a value");
}

#[test]
fn test_fmt_eof_while_parsing_object() {
    let error_code = ErrorCode::EofWhileParsingObject;
    let mut output = String::new();
    
    let result = std::fmt::write(&mut output, format_args!("{}", error_code));
    
    assert!(result.is_ok());
    assert_eq!(output, "EOF while parsing an object");
}

#[test]
fn test_fmt_expected_colon() {
    let error_code = ErrorCode::ExpectedColon;
    let mut output = String::new();
    
    let result = std::fmt::write(&mut output, format_args!("{}", error_code));
    
    assert!(result.is_ok());
    assert_eq!(output, "expected `:`");
}

#[test]
fn test_fmt_invalid_number() {
    let error_code = ErrorCode::InvalidNumber;
    let mut output = String::new();
    
    let result = std::fmt::write(&mut output, format_args!("{}", error_code));
    
    assert!(result.is_ok());
    assert_eq!(output, "invalid number");
}

