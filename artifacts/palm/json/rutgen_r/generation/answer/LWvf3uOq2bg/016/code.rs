// Answer 0

#[test]
fn test_error_code_message() {
    struct ErrorCode {
        msg: String,
    }

    impl std::fmt::Display for ErrorCode {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_str(&self.msg)
        }
    }

    let error_code = ErrorCode { msg: String::from("Some error message") };
    let mut output = String::new();
    assert!(error_code.fmt(&mut output).is_ok());
    assert_eq!(output, "Some error message");
}

#[test]
fn test_error_code_eof_while_parsing_list() {
    #[derive(Debug)]
    enum ErrorCode {
        EofWhileParsingList,
    }

    impl std::fmt::Display for ErrorCode {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                ErrorCode::EofWhileParsingList => f.write_str("EOF while parsing a list"),
            }
        }
    }

    let error_code = ErrorCode::EofWhileParsingList;
    let mut output = String::new();
    assert!(error_code.fmt(&mut output).is_ok());
    assert_eq!(output, "EOF while parsing a list");
}

#[test]
fn test_error_code_expected_some_ident() {
    #[derive(Debug)]
    enum ErrorCode {
        ExpectedSomeIdent,
    }

    impl std::fmt::Display for ErrorCode {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                ErrorCode::ExpectedSomeIdent => f.write_str("expected ident"),
            }
        }
    }

    let error_code = ErrorCode::ExpectedSomeIdent;
    let mut output = String::new();
    assert!(error_code.fmt(&mut output).is_ok());
    assert_eq!(output, "expected ident");
}

#[test]
fn test_error_code_expected_some_value() {
    #[derive(Debug)]
    enum ErrorCode {
        ExpectedSomeValue,
    }

    impl std::fmt::Display for ErrorCode {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                ErrorCode::ExpectedSomeValue => f.write_str("expected value"),
            }
        }
    }

    let error_code = ErrorCode::ExpectedSomeValue;
    let mut output = String::new();
    assert!(error_code.fmt(&mut output).is_ok());
    assert_eq!(output, "expected value");
}

