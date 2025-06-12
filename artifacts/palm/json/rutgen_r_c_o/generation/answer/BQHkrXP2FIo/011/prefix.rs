// Answer 0

#[test]
fn test_classify_number_out_of_range() {
    struct ErrorImpl {
        code: ErrorCode,
        line: usize,
        column: usize,
    }

    struct Error {
        err: Box<ErrorImpl>,
    }

    #[derive(Copy, Clone, PartialEq, Eq)]
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

    let error_impl = ErrorImpl {
        code: ErrorCode::NumberOutOfRange,
        line: 1,
        column: 5,
    };

    let error = Error {
        err: Box::new(error_impl),
    };

    error.classify();
}

