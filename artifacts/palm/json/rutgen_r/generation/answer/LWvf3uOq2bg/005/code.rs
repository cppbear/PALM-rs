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

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErrorCode::Message(msg) => f.write_str(msg),
            ErrorCode::Io(err) => std::fmt::Display::fmt(err, f),
            ErrorCode::EofWhileParsingList => f.write_str("EOF while parsing a list"),
            ErrorCode::EofWhileParsingObject => f.write_str("EOF while parsing an object"),
            ErrorCode::EofWhileParsingString => f.write_str("EOF while parsing a string"),
            ErrorCode::EofWhileParsingValue => f.write_str("EOF while parsing a value"),
            ErrorCode::ExpectedColon => f.write_str("expected `:`"),
            ErrorCode::ExpectedListCommaOrEnd => f.write_str("expected `,` or `]`"),
            ErrorCode::ExpectedObjectCommaOrEnd => f.write_str("expected `,` or `}`"),
            ErrorCode::ExpectedSomeIdent => f.write_str("expected ident"),
            ErrorCode::ExpectedSomeValue => f.write_str("expected value"),
            ErrorCode::ExpectedDoubleQuote => f.write_str("expected `\"`"),
            ErrorCode::InvalidEscape => f.write_str("invalid escape"),
            ErrorCode::InvalidNumber => f.write_str("invalid number"),
            ErrorCode::NumberOutOfRange => f.write_str("number out of range"),
            ErrorCode::InvalidUnicodeCodePoint => f.write_str("invalid unicode code point"),
            ErrorCode::ControlCharacterWhileParsingString => {
                f.write_str("control character (\\u0000-\\u001F) found while parsing a string")
            }
            ErrorCode::KeyMustBeAString => f.write_str("key must be a string"),
            ErrorCode::ExpectedNumericKey => {
                f.write_str("invalid value: expected key to be a number in quotes")
            }
            ErrorCode::FloatKeyMustBeFinite => {
                f.write_str("float key must be finite (got NaN or +/-inf)")
            }
            ErrorCode::LoneLeadingSurrogateInHexEscape => {
                f.write_str("lone leading surrogate in hex escape")
            }
            ErrorCode::TrailingComma => f.write_str("trailing comma"),
            ErrorCode::TrailingCharacters => f.write_str("trailing characters"),
            ErrorCode::UnexpectedEndOfHexEscape => f.write_str("unexpected end of hex escape"),
            ErrorCode::RecursionLimitExceeded => f.write_str("recursion limit exceeded"),
        }
    }
}

#[test]
fn test_error_code_lone_leading_surrogate_in_hex_escape() {
    let error_code = ErrorCode::LoneLeadingSurrogateInHexEscape;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(output, "lone leading surrogate in hex escape");
}

#[test]
fn test_error_code_message() {
    let error_code = ErrorCode::Message("some error occurred".to_string());
    let mut output = String::new();
    let result = write!(&mut output, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(output, "some error occurred");
}

#[test]
fn test_error_code_invalid_escape() {
    let error_code = ErrorCode::InvalidEscape;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(output, "invalid escape");
}

