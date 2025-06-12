// Answer 0

#[test]
fn test_error_code_fmt_expected_double_quote() {
    use std::fmt::{self, Display};

    enum ErrorCode {
        ExpectedDoubleQuote,
        Message(String),
        Io(std::io::Error),
        // other variants omitted for brevity
    }

    impl Display for ErrorCode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                ErrorCode::Message(msg) => f.write_str(msg),
                ErrorCode::ExpectedDoubleQuote => f.write_str("expected `\"`"),
                // other variants omitted for brevity
            }
        }
    }

    let error = ErrorCode::ExpectedDoubleQuote;
    let mut buffer = String::new();
    assert_eq!(error.fmt(&mut fmt::Formatter::new()), Ok(()));
    assert_eq!(buffer, "expected `\"`");
}

#[test]
fn test_error_code_fmt_message() {
    use std::fmt::{self, Display};

    enum ErrorCode {
        ExpectedDoubleQuote,
        Message(String),
        Io(std::io::Error),
        // other variants omitted for brevity
    }

    impl Display for ErrorCode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                ErrorCode::Message(msg) => f.write_str(msg),
                ErrorCode::ExpectedDoubleQuote => f.write_str("expected `\"`"),
                // other variants omitted for brevity
            }
        }
    }

    let error = ErrorCode::Message("Custom error message".to_string());
    let mut buffer = String::new();
    assert_eq!(error.fmt(&mut fmt::Formatter::new()), Ok(()));
    assert_eq!(buffer, "Custom error message");
}

