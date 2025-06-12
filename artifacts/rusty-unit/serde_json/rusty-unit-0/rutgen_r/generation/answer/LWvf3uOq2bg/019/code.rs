// Answer 0

#[test]
fn test_fmt_error_code_expected_colon() {
    use std::fmt::{self, Display, Formatter};

    enum ErrorCode {
        ExpectedColon,
        // ... other variants can be included but are not necessary for this test
    }

    impl Display for ErrorCode {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            match self {
                ErrorCode::ExpectedColon => f.write_str("expected `:`"),
                // ... match arms for other variants can be included but are not necessary for this test
            }
        }
    }

    let error_code = ErrorCode::ExpectedColon;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(output, "expected `:`");
}

#[test]
fn test_fmt_error_code_eof_while_parsing_list() {
    use std::fmt::{self, Display, Formatter};

    enum ErrorCode {
        EofWhileParsingList,
        // ... other variants can be included but are not necessary for this test
    }

    impl Display for ErrorCode {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            match self {
                ErrorCode::EofWhileParsingList => f.write_str("EOF while parsing a list"),
                // ... match arms for other variants can be included but are not necessary for this test
            }
        }
    }

    let error_code = ErrorCode::EofWhileParsingList;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(output, "EOF while parsing a list");
}

