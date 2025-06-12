// Answer 0

fn test_fmt_expected_list_comma_or_end() {
    let error_code = ErrorCode::ExpectedListCommaOrEnd;
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(buffer, "expected `,` or `]`");
}

fn test_fmt_expected_object_comma_or_end() {
    let error_code = ErrorCode::ExpectedObjectCommaOrEnd;
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(buffer, "expected `,` or `}`");
}

fn test_fmt_eof_while_parsing_list() {
    let error_code = ErrorCode::EofWhileParsingList;
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(buffer, "EOF while parsing a list");
}

fn test_fmt_eof_while_parsing_object() {
    let error_code = ErrorCode::EofWhileParsingObject;
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(buffer, "EOF while parsing an object");
}

fn test_fmt_eof_while_parsing_string() {
    let error_code = ErrorCode::EofWhileParsingString;
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(buffer, "EOF while parsing a string");
}

fn test_fmt_eof_while_parsing_value() {
    let error_code = ErrorCode::EofWhileParsingValue;
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(buffer, "EOF while parsing a value");
}

fn test_fmt_expected_some_value() {
    let error_code = ErrorCode::ExpectedSomeValue;
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(buffer, "expected value");
}

fn test_fmt_expected_some_ident() {
    let error_code = ErrorCode::ExpectedSomeIdent;
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(buffer, "expected ident");
}

fn test_fmt_invalid_number() {
    let error_code = ErrorCode::InvalidNumber;
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(buffer, "invalid number");
}

fn test_fmt_unexpected_end_of_hex_escape() {
    let error_code = ErrorCode::UnexpectedEndOfHexEscape;
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(buffer, "unexpected end of hex escape");
}

fn test_fmt_trailing_characters() {
    let error_code = ErrorCode::TrailingCharacters;
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(buffer, "trailing characters");
}

