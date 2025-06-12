// Answer 0

#[test]
fn test_classify_data_category() {
    struct Error {
        code: ErrorCode,
    }

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

    enum Category {
        Io,
        Syntax,
        Data,
        Eof,
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
                _ => Category::Syntax,
            }
        }
    }

    let error_instance = Error {
        code: ErrorCode::Message("This is a test message".to_string()),
    };

    assert_eq!(error_instance.classify(), Category::Data);
}

