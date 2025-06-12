// Answer 0

#[test]
fn test_error_code_trailing_comma() {
    enum ErrorCode {
        TrailingComma,
    }

    use std::fmt::{self, Display};

    impl Display for ErrorCode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                ErrorCode::TrailingComma => f.write_str("trailing comma"),
            }
        }
    }

    let error_code = ErrorCode::TrailingComma;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(output, "trailing comma");
}

#[test]
fn test_error_code_eof_while_parsing_object() {
    enum ErrorCode {
        EofWhileParsingObject,
    }

    use std::fmt::{self, Display};

    impl Display for ErrorCode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                ErrorCode::EofWhileParsingObject => f.write_str("EOF while parsing an object"),
            }
        }
    }

    let error_code = ErrorCode::EofWhileParsingObject;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(output, "EOF while parsing an object");
}

