// Answer 0

#[test]
fn test_classify_syntax_key_must_be_a_string() {
    struct ErrorImpl {
        code: ErrorCode,
        line: usize,
        column: usize,
    }

    struct Error {
        err: Box<ErrorImpl>,
    }

    #[derive(Copy, Clone)]
    enum ErrorCode {
        Message(String),
        Io(std::io::ErrorKind),
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
        code: ErrorCode::KeyMustBeAString,
        line: 1,
        column: 10,
    };
    
    let error = Error {
        err: Box::new(error_impl),
    };

    assert_eq!(error.classify(), Category::Syntax);
}

