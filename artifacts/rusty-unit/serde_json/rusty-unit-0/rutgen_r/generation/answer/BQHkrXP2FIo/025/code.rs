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

struct Error {
    code: ErrorCode,
}

struct CustomError {
    err: Error,
}

impl CustomError {
    pub fn classify(&self) -> Category {
        match self.err.code {
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
fn test_classify_category_data() {
    let error_with_message = CustomError {
        err: Error {
            code: ErrorCode::Message(String::from("An error message")),
        },
    };
    assert_eq!(error_with_message.classify(), Category::Data);
}

#[test]
fn test_classify_category_io() {
    let error_with_io = CustomError {
        err: Error {
            code: ErrorCode::Io(std::io::Error::new(std::io::ErrorKind::Other, "IO error")),
        },
    };
    assert_eq!(error_with_io.classify(), Category::Io);
}

#[test]
fn test_classify_category_eof() {
    let error_with_eof = CustomError {
        err: Error {
            code: ErrorCode::EofWhileParsingList,
        },
    };
    assert_eq!(error_with_eof.classify(), Category::Eof);
}

#[test]
fn test_classify_category_syntax() {
    let error_with_syntax = CustomError {
        err: Error {
            code: ErrorCode::ExpectedColon,
        },
    };
    assert_eq!(error_with_syntax.classify(), Category::Syntax);
}

