// Answer 0

#[test]
fn test_error_debug_format() {
    struct TestError {
        err: Box<ErrorImpl>,
    }

    impl Debug for TestError {
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
        line: 1,
        column: 2,
    };

    let error = TestError { err: Box::new(error_impl) };

    let mut output = String::new();
    let result = write!(&mut output, "{:?}", error);
    assert!(result.is_ok());
    assert_eq!(output, "Error(\"Test error message\", line: 1, column: 2)");
}

#[test]
fn test_error_debug_format_with_io_error() {
    struct TestError {
        err: Box<ErrorImpl>,
    }

    impl Debug for TestError {
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
        code: ErrorCode::Io(std::io::Error::new(std::io::ErrorKind::Other, "I/O error")),
        line: 10,
        column: 20,
    };

    let error = TestError { err: Box::new(error_impl) };

    let mut output = String::new();
    let result = write!(&mut output, "{:?}", error);
    assert!(result.is_ok());
    assert!(output.contains("Error(Io("));
    assert!(output.contains("I/O error"));
    assert_eq!(output.contains("line: 10, column: 20"), true);
}

