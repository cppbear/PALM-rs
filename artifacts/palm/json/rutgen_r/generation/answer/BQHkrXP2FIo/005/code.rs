// Answer 0

#[test]
fn test_classify_lone_leading_surrogate_in_hex_escape() {
    struct Error {
        code: ErrorCode,
    }

    struct TestError {
        err: Error,
    }

    impl TestError {
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

    #[derive(Debug, PartialEq)]
    enum Category {
        Io,
        Syntax,
        Data,
        Eof,
    }

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

    let error = TestError {
        err: Error {
            code: ErrorCode::LoneLeadingSurrogateInHexEscape,
        },
    };

    assert_eq!(error.classify(), Category::Syntax);
}

