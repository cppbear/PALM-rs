// Answer 0

#[test]
fn test_classify_with_trailing_characters() {
    struct TestErrorImpl {
        code: ErrorCode,
        line: usize,
        column: usize,
    }

    struct TestError {
        err: Box<TestErrorImpl>,
    }

    impl Error for TestError {
        pub fn classify(&self) -> Category {
            match self.err.code {
                ErrorCode::TrailingCharacters => Category::Syntax,
                _ => unreachable!(),
            }
        }
    }

    let error_impl = TestErrorImpl {
        code: ErrorCode::TrailingCharacters,
        line: 1,
        column: 1,
    };

    let error = TestError {
        err: Box::new(error_impl),
    };

    error.classify();
}

#[test]
fn test_classify_with_various_syntax_errors() {
    let syntax_errors = vec![
        ErrorCode::ExpectedColon,
        ErrorCode::ExpectedListCommaOrEnd,
        ErrorCode::ExpectedObjectCommaOrEnd,
        ErrorCode::ExpectedSomeIdent,
        ErrorCode::ExpectedSomeValue,
        ErrorCode::ExpectedDoubleQuote,
        ErrorCode::InvalidEscape,
        ErrorCode::InvalidNumber,
        ErrorCode::NumberOutOfRange,
        ErrorCode::InvalidUnicodeCodePoint,
        ErrorCode::ControlCharacterWhileParsingString,
        ErrorCode::KeyMustBeAString,
        ErrorCode::ExpectedNumericKey,
        ErrorCode::FloatKeyMustBeFinite,
        ErrorCode::LoneLeadingSurrogateInHexEscape,
        ErrorCode::TrailingComma,
        ErrorCode::UnexpectedEndOfHexEscape,
            ErrorCode::RecursionLimitExceeded
    ];

    for error_code in syntax_errors {
        let error_impl = TestErrorImpl {
            code: error_code,
            line: 1,
            column: 1,
        };

        let error = TestError {
            err: Box::new(error_impl),
        };

        error.classify();
    }
}

