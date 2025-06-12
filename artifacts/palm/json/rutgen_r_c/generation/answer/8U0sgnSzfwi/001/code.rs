// Answer 0

#[test]
fn test_error_debug_message() {
    struct MockError {
        err: Box<ErrorImpl>,
    }

    impl Debug for MockError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "Error({:?}, line: {}, column: {})",
                self.err.code.to_string(),
                self.err.line,
                self.err.column
            )
        }
    }

    let error_impl = ErrorImpl {
        code: ErrorCode::Message(Box::from("Test error message")),
        line: 10,
        column: 5,
    };

    let error = MockError {
        err: Box::new(error_impl),
    };

    let mut output = String::new();
    {
        let formatter = &mut fmt::Formatter::new(&mut output);
        error.fmt(formatter).unwrap();
    }

    assert_eq!(
        output,
        "Error(\"Test error message\", line: 10, column: 5)"
    );
}

#[test]
fn test_error_debug_eof_while_parsing() {
    struct MockError {
        err: Box<ErrorImpl>,
    }

    impl Debug for MockError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "Error({:?}, line: {}, column: {})",
                self.err.code.to_string(),
                self.err.line,
                self.err.column
            )
        }
    }

    let error_impl = ErrorImpl {
        code: ErrorCode::EofWhileParsingString,
        line: 3,
        column: 12,
    };

    let error = MockError {
        err: Box::new(error_impl),
    };

    let mut output = String::new();
    {
        let formatter = &mut fmt::Formatter::new(&mut output);
        error.fmt(formatter).unwrap();
    }

    assert_eq!(
        output,
        "Error(EofWhileParsingString, line: 3, column: 12)"
    );
}

#[test]
fn test_error_debug_invalid_number() {
    struct MockError {
        err: Box<ErrorImpl>,
    }

    impl Debug for MockError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "Error({:?}, line: {}, column: {})",
                self.err.code.to_string(),
                self.err.line,
                self.err.column
            )
        }
    }

    let error_impl = ErrorImpl {
        code: ErrorCode::InvalidNumber,
        line: 15,
        column: 0,
    };

    let error = MockError {
        err: Box::new(error_impl),
    };

    let mut output = String::new();
    {
        let formatter = &mut fmt::Formatter::new(&mut output);
        error.fmt(formatter).unwrap();
    }

    assert_eq!(
        output,
        "Error(InvalidNumber, line: 15, column: 0)"
    );
}

