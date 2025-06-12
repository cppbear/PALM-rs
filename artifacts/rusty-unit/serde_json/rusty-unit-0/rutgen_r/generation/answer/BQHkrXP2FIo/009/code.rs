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
fn test_classify_control_character_while_parsing_string() {
    let error = Error {
        code: ErrorCode::ControlCharacterWhileParsingString,
    };
    assert_eq!(error.classify(), Category::Syntax);
}

#[test]
fn test_classify_trailing_characters() {
    let error = Error {
        code: ErrorCode::TrailingCharacters,
    };
    assert_eq!(error.classify(), Category::Syntax);
}

#[test]
fn test_classify_eof_while_parsing_value() {
    let error = Error {
        code: ErrorCode::EofWhileParsingValue,
    };
    assert_eq!(error.classify(), Category::Eof);
}

