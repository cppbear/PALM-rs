// Answer 0

#[test]
fn test_error_code_fmt_message() {
    struct TestError;

    impl std::fmt::Display for TestError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_str("Test error")
        }
    }

    enum ErrorCode {
        Message(String),
        Io(TestError),
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

    let error_message = ErrorCode::Message("some message".to_string());
    let mut buffer = String::new();
    assert_eq!(error_message.fmt(&mut buffer), Ok(()));
    assert_eq!(buffer, "some message");
}

#[test]
fn test_error_code_fmt_io_error() {
    struct TestError;

    impl std::fmt::Display for TestError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_str("I/O error occurred")
        }
    }

    enum ErrorCode {
        Message(String),
        Io(TestError),
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

    let io_error = ErrorCode::Io(TestError);
    let mut buffer = String::new();
    assert_eq!(io_error.fmt(&mut buffer), Ok(()));
    assert_eq!(buffer, "I/O error occurred");
}

