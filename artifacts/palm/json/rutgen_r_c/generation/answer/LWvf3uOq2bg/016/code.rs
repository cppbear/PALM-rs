// Answer 0

#[test]
fn test_display_message() {
    let message = "Test error message".to_string().into_boxed_str();
    let error_code = ErrorCode::Message(message);
    let mut output = core::fmt::Formatter::new();
    
    let result = error_code.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "Test error message");
}

#[test]
fn test_display_eof_while_parsing_list() {
    let error_code = ErrorCode::EofWhileParsingList;
    let mut output = core::fmt::Formatter::new();
    
    let result = error_code.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "EOF while parsing a list");
}

#[test]
fn test_display_eof_while_parsing_object() {
    let error_code = ErrorCode::EofWhileParsingObject;
    let mut output = core::fmt::Formatter::new();
    
    let result = error_code.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "EOF while parsing an object");
}

#[test]
fn test_display_eof_while_parsing_string() {
    let error_code = ErrorCode::EofWhileParsingString;
    let mut output = core::fmt::Formatter::new();
    
    let result = error_code.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "EOF while parsing a string");
}

#[test]
fn test_display_eof_while_parsing_value() {
    let error_code = ErrorCode::EofWhileParsingValue;
    let mut output = core::fmt::Formatter::new();
    
    let result = error_code.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "EOF while parsing a value");
}

#[test]
fn test_display_expected_colon() {
    let error_code = ErrorCode::ExpectedColon;
    let mut output = core::fmt::Formatter::new();
    
    let result = error_code.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "expected `:`");
}

#[test]
fn test_display_expected_list_comma_or_end() {
    let error_code = ErrorCode::ExpectedListCommaOrEnd;
    let mut output = core::fmt::Formatter::new();
    
    let result = error_code.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "expected `,` or `]`");
}

#[test]
fn test_display_expected_object_comma_or_end() {
    let error_code = ErrorCode::ExpectedObjectCommaOrEnd;
    let mut output = core::fmt::Formatter::new();
    
    let result = error_code.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "expected `,` or `}`");
}

#[test]
fn test_display_expected_some_ident() {
    let error_code = ErrorCode::ExpectedSomeIdent;
    let mut output = core::fmt::Formatter::new();
    
    let result = error_code.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "expected ident");
}

#[test]
fn test_display_expected_some_value() {
    let error_code = ErrorCode::ExpectedSomeValue;
    let mut output = core::fmt::Formatter::new();
    
    let result = error_code.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "expected value");
}

