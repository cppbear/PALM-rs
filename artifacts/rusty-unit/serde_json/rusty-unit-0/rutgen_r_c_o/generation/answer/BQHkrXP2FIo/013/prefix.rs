// Answer 0

#[test]
fn test_classify_invalid_escape() {
    struct ErrorCode {
        kind: String,
    }

    struct ErrorImpl {
        code: ErrorCode,
        line: usize,
        column: usize,
    }

    struct Error {
        err: Box<ErrorImpl>,
    }

    impl Error {
        pub fn classify(&self) -> Category {
            match self.err.code.kind.as_str() {
                "Message" => Category::Data,
                "Io" => Category::Io,
                "EofWhileParsingList" | "EofWhileParsingObject" | "EofWhileParsingString" | "EofWhileParsingValue" => Category::Eof,
                "ExpectedColon" | "ExpectedListCommaOrEnd" | "ExpectedObjectCommaOrEnd" | "ExpectedSomeIdent" | "ExpectedSomeValue" | "ExpectedDoubleQuote" | "InvalidEscape" | "InvalidNumber" | "NumberOutOfRange" | "InvalidUnicodeCodePoint" | "ControlCharacterWhileParsingString" | "KeyMustBeAString" | "ExpectedNumericKey" | "FloatKeyMustBeFinite" | "LoneLeadingSurrogateInHexEscape" | "TrailingComma" | "TrailingCharacters" | "UnexpectedEndOfHexEscape" | "RecursionLimitExceeded" => Category::Syntax,
                _ => panic!("Unknown error code"),
            }
        }
    }

    let error_code = ErrorCode { kind: String::from("InvalidEscape") };
    let error_impl = ErrorImpl { code: error_code, line: 1, column: 1 };
    let error = Error { err: Box::new(error_impl) };

    error.classify();
}

