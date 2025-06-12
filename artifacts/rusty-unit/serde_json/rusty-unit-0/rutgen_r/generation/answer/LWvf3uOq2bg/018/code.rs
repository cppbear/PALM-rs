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
            ErrorCode::EofWhileParsingList => f.write_str("EOF while parsing a list"),
            ErrorCode::ExpectedListCommaOrEnd => f.write_str("expected `,` or `]`"),
            _ => f.write_str("other error"),
        }
    }
}

#[test]
fn test_expected_list_comma_or_end() {
    let error = ErrorCode::ExpectedListCommaOrEnd;
    let mut output = String::new();
    let result = std::fmt::write(&mut output, format_args!("{}", error));
    assert!(result.is_ok());
    assert_eq!(output, "expected `,` or `]`");
}

#[test]
fn test_eof_while_parsing_list() {
    let error = ErrorCode::EofWhileParsingList;
    let mut output = String::new();
    let result = std::fmt::write(&mut output, format_args!("{}", error));
    assert!(result.is_ok());
    assert_eq!(output, "EOF while parsing a list");
}

