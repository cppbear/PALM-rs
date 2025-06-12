// Answer 0

#[test]
fn test_classify_trailing_comma() {
    struct Error {
        code: ErrorCode,
    }

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

    enum Category {
        Io,
        Syntax,
        Data,
        Eof,
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

    let err = Error {
        code: ErrorCode::TrailingComma,
    };
    
    let category = err.classify();
    assert_eq!(category, Category::Syntax);
}

