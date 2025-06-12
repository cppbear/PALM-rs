// Answer 0

#[test]
fn test_error_code_message() {
    let error_code = ErrorCode::Message(Box::from("Some error occurred"));
    let mut buffer = String::new();
    write!(&mut buffer, "{}", error_code).unwrap();
    assert_eq!(buffer, "Some error occurred");
}

#[test]
fn test_error_code_io() {
    use std::io;
    
    let io_error = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let error_code = ErrorCode::Io(io_error);
    
    let mut buffer = String::new();
    write!(&mut buffer, "{}", error_code).unwrap();
    assert!(buffer.contains("file not found"));
}

#[test]
fn test_error_code_eof_while_parsing_list() {
    let error_code = ErrorCode::EofWhileParsingList;
    let mut buffer = String::new();
    write!(&mut buffer, "{}", error_code).unwrap();
    assert_eq!(buffer, "EOF while parsing a list");
}

#[test]
fn test_error_code_eof_while_parsing_object() {
    let error_code = ErrorCode::EofWhileParsingObject;
    let mut buffer = String::new();
    write!(&mut buffer, "{}", error_code).unwrap();
    assert_eq!(buffer, "EOF while parsing an object");
}

#[test]
fn test_error_code_eof_while_parsing_string() {
    let error_code = ErrorCode::EofWhileParsingString;
    let mut buffer = String::new();
    write!(&mut buffer, "{}", error_code).unwrap();
    assert_eq!(buffer, "EOF while parsing a string");
}

#[test]
fn test_error_code_eof_while_parsing_value() {
    let error_code = ErrorCode::EofWhileParsingValue;
    let mut buffer = String::new();
    write!(&mut buffer, "{}", error_code).unwrap();
    assert_eq!(buffer, "EOF while parsing a value");
}

#[test]
fn test_error_code_expected_colon() {
    let error_code = ErrorCode::ExpectedColon;
    let mut buffer = String::new();
    write!(&mut buffer, "{}", error_code).unwrap();
    assert_eq!(buffer, "expected `:`");
}

#[test]
fn test_error_code_expected_list_comma_or_end() {
    let error_code = ErrorCode::ExpectedListCommaOrEnd;
    let mut buffer = String::new();
    write!(&mut buffer, "{}", error_code).unwrap();
    assert_eq!(buffer, "expected `,` or `]`");
}

#[test]
fn test_error_code_expected_object_comma_or_end() {
    let error_code = ErrorCode::ExpectedObjectCommaOrEnd;
    let mut buffer = String::new();
    write!(&mut buffer, "{}", error_code).unwrap();
    assert_eq!(buffer, "expected `,` or `}`");
}

#[test]
fn test_error_code_expected_some_ident() {
    let error_code = ErrorCode::ExpectedSomeIdent;
    let mut buffer = String::new();
    write!(&mut buffer, "{}", error_code).unwrap();
    assert_eq!(buffer, "expected ident");
}

#[test]
fn test_error_code_expected_some_value() {
    let error_code = ErrorCode::ExpectedSomeValue;
    let mut buffer = String::new();
    write!(&mut buffer, "{}", error_code).unwrap();
    assert_eq!(buffer, "expected value");
}

#[test]
fn test_error_code_expected_double_quote() {
    let error_code = ErrorCode::ExpectedDoubleQuote;
    let mut buffer = String::new();
    write!(&mut buffer, "{}", error_code).unwrap();
    assert_eq!(buffer, "expected `\"`");
}

#[test]
fn test_error_code_invalid_escape() {
    let error_code = ErrorCode::InvalidEscape;
    let mut buffer = String::new();
    write!(&mut buffer, "{}", error_code).unwrap();
    assert_eq!(buffer, "invalid escape");
}

#[test]
fn test_error_code_invalid_number() {
    let error_code = ErrorCode::InvalidNumber;
    let mut buffer = String::new();
    write!(&mut buffer, "{}", error_code).unwrap();
    assert_eq!(buffer, "invalid number");
}

#[test]
fn test_error_code_number_out_of_range() {
    let error_code = ErrorCode::NumberOutOfRange;
    let mut buffer = String::new();
    write!(&mut buffer, "{}", error_code).unwrap();
    assert_eq!(buffer, "number out of range");
}

#[test]
fn test_error_code_invalid_unicode_code_point() {
    let error_code = ErrorCode::InvalidUnicodeCodePoint;
    let mut buffer = String::new();
    write!(&mut buffer, "{}", error_code).unwrap();
    assert_eq!(buffer, "invalid unicode code point");
}

#[test]
fn test_error_code_control_character_while_parsing_string() {
    let error_code = ErrorCode::ControlCharacterWhileParsingString;
    let mut buffer = String::new();
    write!(&mut buffer, "{}", error_code).unwrap();
    assert_eq!(buffer, "control character (\\u0000-\\u001F) found while parsing a string");
}

#[test]
fn test_error_code_key_must_be_a_string() {
    let error_code = ErrorCode::KeyMustBeAString;
    let mut buffer = String::new();
    write!(&mut buffer, "{}", error_code).unwrap();
    assert_eq!(buffer, "key must be a string");
}

#[test]
fn test_error_code_expected_numeric_key() {
    let error_code = ErrorCode::ExpectedNumericKey;
    let mut buffer = String::new();
    write!(&mut buffer, "{}", error_code).unwrap();
    assert_eq!(buffer, "invalid value: expected key to be a number in quotes");
}

#[test]
fn test_error_code_float_key_must_be_finite() {
    let error_code = ErrorCode::FloatKeyMustBeFinite;
    let mut buffer = String::new();
    write!(&mut buffer, "{}", error_code).unwrap();
    assert_eq!(buffer, "float key must be finite (got NaN or +/-inf)");
}

#[test]
fn test_error_code_lone_leading_surrogate_in_hex_escape() {
    let error_code = ErrorCode::LoneLeadingSurrogateInHexEscape;
    let mut buffer = String::new();
    write!(&mut buffer, "{}", error_code).unwrap();
    assert_eq!(buffer, "lone leading surrogate in hex escape");
}

#[test]
fn test_error_code_trailing_comma() {
    let error_code = ErrorCode::TrailingComma;
    let mut buffer = String::new();
    write!(&mut buffer, "{}", error_code).unwrap();
    assert_eq!(buffer, "trailing comma");
}

#[test]
fn test_error_code_trailing_characters() {
    let error_code = ErrorCode::TrailingCharacters;
    let mut buffer = String::new();
    write!(&mut buffer, "{}", error_code).unwrap();
    assert_eq!(buffer, "trailing characters");
}

#[test]
fn test_error_code_unexpected_end_of_hex_escape() {
    let error_code = ErrorCode::UnexpectedEndOfHexEscape;
    let mut buffer = String::new();
    write!(&mut buffer, "{}", error_code).unwrap();
    assert_eq!(buffer, "unexpected end of hex escape");
}

#[test]
fn test_error_code_recursion_limit_exceeded() {
    let error_code = ErrorCode::RecursionLimitExceeded;
    let mut buffer = String::new();
    write!(&mut buffer, "{}", error_code).unwrap();
    assert_eq!(buffer, "recursion limit exceeded");
}

