// Answer 0

#[test]
fn test_error_code_fmt_unexpected_end_of_hex_escape() {
    use std::fmt::{self, Display};

    #[derive(Debug)]
    enum ErrorCode {
        UnexpectedEndOfHexEscape,
    }

    impl Display for ErrorCode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                ErrorCode::UnexpectedEndOfHexEscape => f.write_str("unexpected end of hex escape"),
            }
        }
    }

    let error = ErrorCode::UnexpectedEndOfHexEscape;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    
    assert!(result.is_ok());
    assert_eq!(output, "unexpected end of hex escape");
}

#[test]
fn test_error_code_fmt_unexpected_end_of_hex_escape_repeat() {
    use std::fmt::{self, Display};

    #[derive(Debug)]
    enum ErrorCode {
        UnexpectedEndOfHexEscape,
    }

    impl Display for ErrorCode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                ErrorCode::UnexpectedEndOfHexEscape => f.write_str("unexpected end of hex escape"),
            }
        }
    }

    let error = ErrorCode::UnexpectedEndOfHexEscape;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    
    assert!(result.is_ok());
    assert_eq!(output, "unexpected end of hex escape");
}

