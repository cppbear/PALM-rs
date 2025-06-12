// Answer 0

#[test]
fn test_classify_eof_while_parsing_object() {
    struct MockErrorImpl {
        code: ErrorCode,
        line: usize,
        column: usize,
    }

    struct MockError {
        err: Box<MockErrorImpl>,
    }

    impl MockError {
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

    let error_impl = MockErrorImpl {
        code: ErrorCode::EofWhileParsingObject,
        line: 1,
        column: 5,
    };

    let error = MockError {
        err: Box::new(error_impl),
    };

    assert_eq!(error.classify(), Category::Eof);
}

#[test]
fn test_classify_eof_while_parsing_value() {
    struct MockErrorImpl {
        code: ErrorCode,
        line: usize,
        column: usize,
    }

    struct MockError {
        err: Box<MockErrorImpl>,
    }

    impl MockError {
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

    let error_impl = MockErrorImpl {
        code: ErrorCode::EofWhileParsingValue,
        line: 2,
        column: 10,
    };

    let error = MockError {
        err: Box::new(error_impl),
    };

    assert_eq!(error.classify(), Category::Eof);
}

#[test]
fn test_classify_eof_while_parsing_string() {
    struct MockErrorImpl {
        code: ErrorCode,
        line: usize,
        column: usize,
    }

    struct MockError {
        err: Box<MockErrorImpl>,
    }

    impl MockError {
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

    let error_impl = MockErrorImpl {
        code: ErrorCode::EofWhileParsingString,
        line: 3,
        column: 15,
    };

    let error = MockError {
        err: Box::new(error_impl),
    };

    assert_eq!(error.classify(), Category::Eof);
}

#[test]
fn test_classify_eof_while_parsing_list() {
    struct MockErrorImpl {
        code: ErrorCode,
        line: usize,
        column: usize,
    }

    struct MockError {
        err: Box<MockErrorImpl>,
    }

    impl MockError {
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

    let error_impl = MockErrorImpl {
        code: ErrorCode::EofWhileParsingList,
        line: 4,
        column: 20,
    };

    let error = MockError {
        err: Box::new(error_impl),
    };

    assert_eq!(error.classify(), Category::Eof);
}

