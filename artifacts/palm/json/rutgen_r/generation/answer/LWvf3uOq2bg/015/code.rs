// Answer 0

#[test]
fn test_error_code_fmt_message() {
    struct ErrorCode {
        kind: ErrorKind,
    }

    enum ErrorKind {
        Message(String),
        EofWhileParsingValue,
        ExpectedSomeValue,
    }

    impl std::fmt::Display for ErrorCode {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match &self.kind {
                ErrorKind::Message(msg) => f.write_str(msg),
                ErrorKind::EofWhileParsingValue => f.write_str("EOF while parsing a value"),
                ErrorKind::ExpectedSomeValue => f.write_str("expected value"),
            }
        }
    }

    let error_message = "An error occurred";
    let error_code_msg = ErrorCode { kind: ErrorKind::Message(error_message.to_string()) };
    let error_code_eof = ErrorCode { kind: ErrorKind::EofWhileParsingValue };
    let error_code_expected_value = ErrorCode { kind: ErrorKind::ExpectedSomeValue };
    
    let mut output = String::new();
    assert!(error_code_msg.fmt(&mut output).is_ok());
    assert_eq!(output, error_message);
    
    output.clear();
    assert!(error_code_eof.fmt(&mut output).is_ok());
    assert_eq!(output, "EOF while parsing a value");
    
    output.clear();
    assert!(error_code_expected_value.fmt(&mut output).is_ok());
    assert_eq!(output, "expected value");
}

