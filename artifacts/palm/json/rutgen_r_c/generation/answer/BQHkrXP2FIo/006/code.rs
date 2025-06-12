// Answer 0

#[test]
fn test_classify_float_key_must_be_finite() {
    struct ErrorImpl {
        code: ErrorCode,
        line: usize,
        column: usize,
    }

    struct Error {
        err: Box<ErrorImpl>,
    }

    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum ErrorCode {
        FloatKeyMustBeFinite,
        Message(String),
        Io(io::Error),
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
        LoneLeadingSurrogateInHexEscape,
        TrailingComma,
        TrailingCharacters,
        UnexpectedEndOfHexEscape,
        RecursionLimitExceeded,
    }

    let error_impl = ErrorImpl {
        code: ErrorCode::FloatKeyMustBeFinite,
        line: 0,
        column: 0,
    };

    let error = Error {
        err: Box::new(error_impl),
    };

    assert_eq!(error.classify(), Category::Syntax);
}

