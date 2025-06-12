// Answer 0

#[test]
fn test_fmt_eof_while_parsing_object() {
    use std::fmt::{self, Display};

    #[derive(Debug)]
    enum ErrorCode {
        EofWhileParsingObject,
        // other variants are omitted for brevity
    }

    impl Display for ErrorCode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                ErrorCode::EofWhileParsingObject => f.write_str("EOF while parsing an object"),
                // other matches omitted for brevity
            }
        }
    }

    let error_code = ErrorCode::EofWhileParsingObject;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error_code);
    
    assert!(result.is_ok());
    assert_eq!(output, "EOF while parsing an object");
}

#[test]
fn test_fmt_eof_while_parsing_list() {
    use std::fmt::{self, Display};

    #[derive(Debug)]
    enum ErrorCode {
        EofWhileParsingList,
        // other variants are omitted for brevity
    }

    impl Display for ErrorCode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                ErrorCode::EofWhileParsingList => f.write_str("EOF while parsing a list"),
                // other matches omitted for brevity
            }
        }
    }

    let error_code = ErrorCode::EofWhileParsingList;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error_code);
    
    assert!(result.is_ok());
    assert_eq!(output, "EOF while parsing a list");
}

#[test]
fn test_fmt_invalid_number() {
    use std::fmt::{self, Display};

    #[derive(Debug)]
    enum ErrorCode {
        InvalidNumber,
        // other variants are omitted for brevity
    }

    impl Display for ErrorCode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                ErrorCode::InvalidNumber => f.write_str("invalid number"),
                // other matches omitted for brevity
            }
        }
    }

    let error_code = ErrorCode::InvalidNumber;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error_code);
    
    assert!(result.is_ok());
    assert_eq!(output, "invalid number");
}

