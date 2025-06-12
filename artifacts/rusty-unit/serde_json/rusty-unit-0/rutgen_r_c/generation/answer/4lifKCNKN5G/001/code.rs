// Answer 0

#[test]
fn test_error_line() {
    struct ErrorImpl {
        code: ErrorCode,
        line: usize,
        column: usize,
    }

    struct Error {
        err: Box<ErrorImpl>,
    }

    impl Error {
        pub fn line(&self) -> usize {
            self.err.line
        }
    }

    // Test with typical line number
    let error = Error {
        err: Box::new(ErrorImpl { code: ErrorCode::Syntax, line: 5, column: 10 }),
    };
    assert_eq!(error.line(), 5);

    // Test with line number 1 (first line)
    let error = Error {
        err: Box::new(ErrorImpl { code: ErrorCode::Syntax, line: 1, column: 0 }),
    };
    assert_eq!(error.line(), 1);

    // Test with a large line number
    let error = Error {
        err: Box::new(ErrorImpl { code: ErrorCode::Data, line: 1000, column: 20 }),
    };
    assert_eq!(error.line(), 1000);
}

