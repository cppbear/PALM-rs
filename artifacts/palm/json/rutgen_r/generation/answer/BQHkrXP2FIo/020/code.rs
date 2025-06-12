// Answer 0

#[derive(Clone, Copy, Debug)]
enum ErrorCode {
    Message(String),
    Io(String),
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

#[derive(Clone, Copy, Debug)]
enum Category {
    Io,
    Syntax,
    Data,
    Eof,
}

struct Error {
    code: ErrorCode,
}

impl Error {
    fn classify(&self) -> Category {
        match self.code {
            ErrorCode::Message(_) => Category::Data,
            ErrorCode::Io(_) => Category::Io,
            ErrorCode::EofWhileParsingList
            | ErrorCode::EofWhileParsingObject
            | ErrorCode::EofWhileParsingString
            | ErrorCode::EofWhileParsingValue => Category::Eof,
            ErrorCode::ExpectedColon
            | ErrorCode::ExpectedListCommaOrEnd
            | ErrorCode::ExpectedObjectCommaOrEnd
            | ErrorCode::ExpectedSomeIdent
            | ErrorCode::ExpectedSomeValue
            | ErrorCode::ExpectedDoubleQuote
            | ErrorCode::InvalidEscape
            | ErrorCode::InvalidNumber
            | ErrorCode::NumberOutOfRange
            | ErrorCode::InvalidUnicodeCodePoint
            | ErrorCode::ControlCharacterWhileParsingString
            | ErrorCode::KeyMustBeAString
            | ErrorCode::ExpectedNumericKey
            | ErrorCode::FloatKeyMustBeFinite
            | ErrorCode::LoneLeadingSurrogateInHexEscape
            | ErrorCode::TrailingComma
            | ErrorCode::TrailingCharacters
            | ErrorCode::UnexpectedEndOfHexEscape
            | ErrorCode::RecursionLimitExceeded => Category::Syntax,
        }
    }
}

#[test]
fn test_classify_eof_while_parsing_value() {
    let error = Error {
        code: ErrorCode::EofWhileParsingValue,
    };
    assert_eq!(error.classify(), Category::Eof);
}

#[test]
fn test_classify_eof_while_parsing_object() {
    let error = Error {
        code: ErrorCode::EofWhileParsingObject,
    };
    assert_eq!(error.classify(), Category::Eof);
}

#[test]
fn test_classify_eof_while_parsing_string() {
    let error = Error {
        code: ErrorCode::EofWhileParsingString,
    };
    assert_eq!(error.classify(), Category::Eof);
}

#[test]
fn test_classify_eof_while_parsing_list() {
    let error = Error {
        code: ErrorCode::EofWhileParsingList,
    };
    assert_eq!(error.classify(), Category::Eof);
}

