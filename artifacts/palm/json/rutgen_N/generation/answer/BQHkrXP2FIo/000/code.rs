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

#[derive(Debug)]
enum Category {
    Io,
    Syntax,
    Data,
    Eof,
}

#[derive(Debug)]
struct Error {
    code: ErrorCode,
}

impl Error {
    pub fn classify(&self) -> Category {
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
fn test_classify_data_category() {
    let error = Error {
        code: ErrorCode::Message("Test Message".to_string()),
    };
    assert_eq!(error.classify(), Category::Data);
}

#[test]
fn test_classify_io_category() {
    let error = Error {
        code: ErrorCode::Io(std::io::Error::new(std::io::ErrorKind::Other, "IO error")),
    };
    assert_eq!(error.classify(), Category::Io);
}

#[test]
fn test_classify_eof_category() {
    let error = Error {
        code: ErrorCode::EofWhileParsingList,
    };
    assert_eq!(error.classify(), Category::Eof);
}

#[test]
fn test_classify_syntax_category() {
    let error = Error {
        code: ErrorCode::ExpectedColon,
    };
    assert_eq!(error.classify(), Category::Syntax);
}

