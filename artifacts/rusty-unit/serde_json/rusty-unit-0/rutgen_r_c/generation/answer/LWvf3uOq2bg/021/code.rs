// Answer 0

#[test]
fn test_error_code_eof_while_parsing_list() {
    use std::fmt::Write;

    let code = ErrorCode::EofWhileParsingList;
    let mut output = String::new();
    let result = code.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "EOF while parsing a list");
}

#[test]
fn test_error_code_eof_while_parsing_object() {
    use std::fmt::Write;

    let code = ErrorCode::EofWhileParsingObject;
    let mut output = String::new();
    let result = code.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "EOF while parsing an object");
}

#[test]
fn test_error_code_eof_while_parsing_string() {
    use std::fmt::Write;

    let code = ErrorCode::EofWhileParsingString;
    let mut output = String::new();
    let result = code.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "EOF while parsing a string");
}

#[test]
fn test_error_code_eof_while_parsing_value() {
    use std::fmt::Write;

    let code = ErrorCode::EofWhileParsingValue;
    let mut output = String::new();
    let result = code.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "EOF while parsing a value");
}

#[test]
fn test_error_code_expected_colon() {
    use std::fmt::Write;

    let code = ErrorCode::ExpectedColon;
    let mut output = String::new();
    let result = code.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "expected `:`");
}

#[test]
fn test_error_code_expected_list_comma_or_end() {
    use std::fmt::Write;

    let code = ErrorCode::ExpectedListCommaOrEnd;
    let mut output = String::new();
    let result = code.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "expected `,` or `]`");
}

#[test]
fn test_error_code_expected_object_comma_or_end() {
    use std::fmt::Write;

    let code = ErrorCode::ExpectedObjectCommaOrEnd;
    let mut output = String::new();
    let result = code.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "expected `,` or `}`");
}

#[test]
fn test_error_code_expected_some_ident() {
    use std::fmt::Write;

    let code = ErrorCode::ExpectedSomeIdent;
    let mut output = String::new();
    let result = code.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "expected ident");
}

