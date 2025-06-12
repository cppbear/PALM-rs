// Answer 0

#[test]
fn test_errorcode_eof_while_parsing_list() {
    use core::fmt::Write;
    
    let error = ErrorCode::EofWhileParsingList;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "EOF while parsing a list");
}

#[test]
fn test_errorcode_eof_while_parsing_object() {
    use core::fmt::Write;
    
    let error = ErrorCode::EofWhileParsingObject;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "EOF while parsing an object");
}

#[test]
fn test_errorcode_eof_while_parsing_string() {
    use core::fmt::Write;
    
    let error = ErrorCode::EofWhileParsingString;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "EOF while parsing a string");
}

#[test]
fn test_errorcode_expected_colon() {
    use core::fmt::Write;
    
    let error = ErrorCode::ExpectedColon;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "expected `:`");
}

#[test]
fn test_errorcode_expected_list_comma_or_end() {
    use core::fmt::Write;
    
    let error = ErrorCode::ExpectedListCommaOrEnd;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "expected `,` or `]`");
}

#[test]
fn test_errorcode_expected_object_comma_or_end() {
    use core::fmt::Write;
    
    let error = ErrorCode::ExpectedObjectCommaOrEnd;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "expected `,` or `}`");
}

