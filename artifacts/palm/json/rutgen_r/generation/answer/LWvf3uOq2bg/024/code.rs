// Answer 0

#[test]
fn test_error_code_message() {
    struct ErrorCode;
    impl std::fmt::Display for ErrorCode {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_str("Some error message")
        }
    }

    let error = ErrorCode::Message(String::from("An error occurred"));
    let mut output = String::new();
    let result = error.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "An error occurred");
}

#[test]
fn test_error_code_io() {
    use std::io;

    let io_error = io::Error::new(io::ErrorKind::Other, "IO error occurred");
    let error = ErrorCode::Io(io_error);
    let mut output = String::new();
    let result = error.fmt(&mut output);
    assert!(result.is_ok());
    assert!(output.contains("IO error occurred"));
}

#[test]
fn test_error_code_eof_while_parsing_list() {
    let error = ErrorCode::EofWhileParsingList;
    let mut output = String::new();
    let result = error.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "EOF while parsing a list");
}

#[test]
fn test_error_code_eof_while_parsing_object() {
    let error = ErrorCode::EofWhileParsingObject;
    let mut output = String::new();
    let result = error.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "EOF while parsing an object");
}

#[test]
fn test_error_code_eof_while_parsing_string() {
    let error = ErrorCode::EofWhileParsingString;
    let mut output = String::new();
    let result = error.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "EOF while parsing a string");
}

#[test]
fn test_error_code_eof_while_parsing_value() {
    let error = ErrorCode::EofWhileParsingValue;
    let mut output = String::new();
    let result = error.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "EOF while parsing a value");
}

#[test]
fn test_error_code_expected_colon() {
    let error = ErrorCode::ExpectedColon;
    let mut output = String::new();
    let result = error.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "expected `:`");
}

#[test]
fn test_error_code_expected_list_comma_or_end() {
    let error = ErrorCode::ExpectedListCommaOrEnd;
    let mut output = String::new();
    let result = error.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "expected `,` or `]`");
}

#[test]
fn test_error_code_expected_object_comma_or_end() {
    let error = ErrorCode::ExpectedObjectCommaOrEnd;
    let mut output = String::new();
    let result = error.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "expected `,` or `}`");
}

#[test]
fn test_error_code_expected_some_ident() {
    let error = ErrorCode::ExpectedSomeIdent;
    let mut output = String::new();
    let result = error.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "expected ident");
}

#[test]
fn test_error_code_expected_some_value() {
    let error = ErrorCode::ExpectedSomeValue;
    let mut output = String::new();
    let result = error.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "expected value");
}

#[test]
fn test_error_code_expected_double_quote() {
    let error = ErrorCode::ExpectedDoubleQuote;
    let mut output = String::new();
    let result = error.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "expected `\"`");
}

#[test]
fn test_error_code_invalid_escape() {
    let error = ErrorCode::InvalidEscape;
    let mut output = String::new();
    let result = error.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "invalid escape");
}

#[test]
fn test_error_code_invalid_number() {
    let error = ErrorCode::InvalidNumber;
    let mut output = String::new();
    let result = error.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "invalid number");
}

#[test]
fn test_error_code_number_out_of_range() {
    let error = ErrorCode::NumberOutOfRange;
    let mut output = String::new();
    let result = error.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "number out of range");
}

#[test]
fn test_error_code_invalid_unicode_code_point() {
    let error = ErrorCode::InvalidUnicodeCodePoint;
    let mut output = String::new();
    let result = error.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "invalid unicode code point");
}

#[test]
fn test_error_code_control_character_while_parsing_string() {
    let error = ErrorCode::ControlCharacterWhileParsingString;
    let mut output = String::new();
    let result = error.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "control character (\\u0000-\\u001F) found while parsing a string");
}

#[test]
fn test_error_code_key_must_be_a_string() {
    let error = ErrorCode::KeyMustBeAString;
    let mut output = String::new();
    let result = error.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "key must be a string");
}

#[test]
fn test_error_code_expected_numeric_key() {
    let error = ErrorCode::ExpectedNumericKey;
    let mut output = String::new();
    let result = error.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "invalid value: expected key to be a number in quotes");
}

#[test]
fn test_error_code_float_key_must_be_finite() {
    let error = ErrorCode::FloatKeyMustBeFinite;
    let mut output = String::new();
    let result = error.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "float key must be finite (got NaN or +/-inf)");
}

#[test]
fn test_error_code_lone_leading_surrogate_in_hex_escape() {
    let error = ErrorCode::LoneLeadingSurrogateInHexEscape;
    let mut output = String::new();
    let result = error.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "lone leading surrogate in hex escape");
}

#[test]
fn test_error_code_trailing_comma() {
    let error = ErrorCode::TrailingComma;
    let mut output = String::new();
    let result = error.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "trailing comma");
}

#[test]
fn test_error_code_trailing_characters() {
    let error = ErrorCode::TrailingCharacters;
    let mut output = String::new();
    let result = error.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "trailing characters");
}

#[test]
fn test_error_code_unexpected_end_of_hex_escape() {
    let error = ErrorCode::UnexpectedEndOfHexEscape;
    let mut output = String::new();
    let result = error.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "unexpected end of hex escape");
}

#[test]
fn test_error_code_recursion_limit_exceeded() {
    let error = ErrorCode::RecursionLimitExceeded;
    let mut output = String::new();
    let result = error.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "recursion limit exceeded");
}

