// Answer 0

#[test]
fn test_eof_while_parsing_object() {
    let error_code = ErrorCode::EofWhileParsingObject;
    let mut buffer = String::new();
    let result = error_code.fmt(&mut buffer);
    assert!(result.is_ok());
    assert_eq!(buffer, "EOF while parsing an object");
}

#[test]
fn test_eof_while_parsing_list() {
    let error_code = ErrorCode::EofWhileParsingList;
    let mut buffer = String::new();
    let result = error_code.fmt(&mut buffer);
    assert!(result.is_ok());
    assert_eq!(buffer, "EOF while parsing a list");
}

#[test]
fn test_expected_colon() {
    let error_code = ErrorCode::ExpectedColon;
    let mut buffer = String::new();
    let result = error_code.fmt(&mut buffer);
    assert!(result.is_ok());
    assert_eq!(buffer, "expected `:`");
}

#[test]
fn test_expected_list_comma_or_end() {
    let error_code = ErrorCode::ExpectedListCommaOrEnd;
    let mut buffer = String::new();
    let result = error_code.fmt(&mut buffer);
    assert!(result.is_ok());
    assert_eq!(buffer, "expected `,` or `]");
}

#[test]
fn test_expected_object_comma_or_end() {
    let error_code = ErrorCode::ExpectedObjectCommaOrEnd;
    let mut buffer = String::new();
    let result = error_code.fmt(&mut buffer);
    assert!(result.is_ok());
    assert_eq!(buffer, "expected `,` or `}`");
}

