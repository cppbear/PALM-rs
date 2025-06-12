// Answer 0

#[test]
fn test_error_code_fmt_expected_double_quote() {
    use std::fmt;

    #[derive(Debug)]
    enum ErrorCode {
        ExpectedDoubleQuote,
    }

    impl fmt::Display for ErrorCode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                ErrorCode::ExpectedDoubleQuote => f.write_str("expected `\"`"),
            }
        }
    }

    let err_code = ErrorCode::ExpectedDoubleQuote;
    let mut output = String::new();
    let result = fmt::write(&mut output, format_args!("{}", err_code));
    
    assert!(result.is_ok());
    assert_eq!(output, "expected `\"`");
}

#[test]
fn test_error_code_fmt_multiple_conditions() {
    use std::fmt;

    #[derive(Debug)]
    enum ErrorCode {
        ExpectedDoubleQuote,
        EofWhileParsingString,
    }

    impl fmt::Display for ErrorCode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                ErrorCode::ExpectedDoubleQuote => f.write_str("expected `\"`"),
                ErrorCode::EofWhileParsingString => f.write_str("EOF while parsing a string"),
            }
        }
    }

    let err_code = ErrorCode::ExpectedDoubleQuote;
    let mut output = String::new();
    let result = fmt::write(&mut output, format_args!("{}", err_code));
    
    assert!(result.is_ok());
    assert_eq!(output, "expected `\"`");
    
    output.clear();
    let err_code_eof = ErrorCode::EofWhileParsingString;
    let result_eof = fmt::write(&mut output, format_args!("{}", err_code_eof));
    
    assert!(result_eof.is_ok());
    assert_eq!(output, "EOF while parsing a string");
}

