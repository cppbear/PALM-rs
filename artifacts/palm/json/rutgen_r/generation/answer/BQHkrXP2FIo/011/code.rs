// Answer 0

#[test]
fn test_classify_number_out_of_range() {
    struct Error {
        code: ErrorCode,
    }

    struct MyError {
        err: Error,
    }

    impl MyError {
        fn classify(&self) -> Category {
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

    enum ErrorCode {
        Io(std::io::Error),
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

    let error_instance = MyError {
        err: Error {
            code: ErrorCode::NumberOutOfRange,
        },
    };

    let result = error_instance.classify();
    assert_eq!(result, Category::Syntax);
}

