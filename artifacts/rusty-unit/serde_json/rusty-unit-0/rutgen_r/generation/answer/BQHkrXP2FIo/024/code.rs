// Answer 0

#[test]
fn test_classify_io_error() {
    struct Error {
        code: ErrorCode,
    }

    struct MyError {
        err: Error,
    }

    enum ErrorCode {
        Io(String),
        Message(String),
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

    enum Category {
        Io,
        Syntax,
        Data,
        Eof,
    }

    impl MyError {
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

    let io_error = MyError {
        err: Error {
            code: ErrorCode::Io(String::from("Unable to read file")),
        },
    };

    assert_eq!(io_error.classify(), Category::Io);
}

