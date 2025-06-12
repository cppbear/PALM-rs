// Answer 0

#[test]
fn test_fmt_error_code_trailing_characters() {
    struct TestErrorCode;

    impl std::fmt::Display for TestErrorCode {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_str("trailing characters")
        }
    }

    enum ErrorCode {
        Message(&'static str),
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

    impl std::fmt::Display for ErrorCode {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                ErrorCode::TrailingCharacters => f.write_str("trailing characters"),
                _ => Ok(()), // We limit the scope here to only test "TrailingCharacters"
            }
        }
    }

    let error_code = ErrorCode::TrailingCharacters;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error_code);

    assert!(result.is_ok());
    assert_eq!(output, "trailing characters");
}

#[test]
fn test_fmt_error_code_multiple_trailing_characters() {
    struct TestErrorCode;

    impl std::fmt::Display for TestErrorCode {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_str("trailing characters")
        }
    }

    enum ErrorCode {
        TrailingCharacters,
    }

    impl std::fmt::Display for ErrorCode {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                ErrorCode::TrailingCharacters => f.write_str("trailing characters"),
            }
        }
    }

    let error_code = ErrorCode::TrailingCharacters;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error_code);

    assert!(result.is_ok());
    assert_eq!(output, "trailing characters");
}

