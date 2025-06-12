// Answer 0

#[test]
fn test_classify_invalid_number() {
    struct MockErrorImpl {
        code: ErrorCode,
        line: usize,
        column: usize,
    }

    struct MockError {
        err: Box<MockErrorImpl>,
    }

    impl MockError {
        pub fn new(code: ErrorCode, line: usize, column: usize) -> Self {
            Self {
                err: Box::new(MockErrorImpl { code, line, column }),
            }
        }

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

    let error = MockError::new(ErrorCode::InvalidNumber, 10, 5);
    assert_eq!(error.classify(), Category::Syntax);
}

