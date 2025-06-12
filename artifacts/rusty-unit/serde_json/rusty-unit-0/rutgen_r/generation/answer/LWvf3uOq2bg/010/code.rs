// Answer 0

#[derive(Debug)]
enum ErrorCode {
    Message(String),
    Io(std::io::Error),
    EofWhileParsingList,
    EofWhileParsingObject,
    EofWhileParsingString,
    EofWhileParsingValue,
    ExpectedColon,
    ExpectedListCommaOrEnd,
    ExpectedObjectCommaOrEnd,
    ExpectedSomeIdent,
    ExpectedSomeValue,
    ExpectedDoubleQuote,
    InvalidEscape,
    InvalidNumber,
    NumberOutOfRange,
    InvalidUnicodeCodePoint,
    ControlCharacterWhileParsingString,
    KeyMustBeAString,
    ExpectedNumericKey,
    FloatKeyMustBeFinite,
    LoneLeadingSurrogateInHexEscape,
    TrailingComma,
    TrailingCharacters,
    UnexpectedEndOfHexEscape,
    RecursionLimitExceeded,
}

#[test]
fn test_fmt_message() {
    let error = ErrorCode::Message("Test message".to_string());
    let mut output = String::new();
    let result = std::fmt::write(&mut output, format_args!("{}", error));
    assert!(result.is_ok());
    assert_eq!(output, "Test message");
}

#[test]
fn test_fmt_invalid_unicode_code_point() {
    let error = ErrorCode::InvalidUnicodeCodePoint;
    let mut output = String::new();
    let result = std::fmt::write(&mut output, format_args!("{}", error));
    assert!(result.is_ok());
    assert_eq!(output, "invalid unicode code point");
}

#[test]
fn test_fmt_control_character_while_parsing_string() {
    let error = ErrorCode::ControlCharacterWhileParsingString;
    let mut output = String::new();
    let result = std::fmt::write(&mut output, format_args!("{}", error));
    assert!(result.is_ok());
    assert_eq!(output, "control character (\\u0000-\\u001F) found while parsing a string");
}

#[test]
fn test_fmt_number_out_of_range() {
    let error = ErrorCode::NumberOutOfRange;
    let mut output = String::new();
    let result = std::fmt::write(&mut output, format_args!("{}", error));
    assert!(result.is_ok());
    assert_eq!(output, "number out of range");
}

