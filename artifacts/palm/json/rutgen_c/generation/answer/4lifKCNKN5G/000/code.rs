// Answer 0

#[test]
fn test_error_line() {
    struct TestError {
        err: Box<ErrorImpl>,
    }

    impl Error {
        pub fn new(code: ErrorCode, line: usize, column: usize) -> Self {
            Self {
                err: Box::new(ErrorImpl { code, line, column }),
            }
        }
    }

    let error_instance = TestError::new(ErrorCode::CustomError, 5, 10);
    assert_eq!(error_instance.line(), 5);
}

#[test]
fn test_error_line_boundary() {
    struct TestError {
        err: Box<ErrorImpl>,
    }

    impl Error {
        pub fn new(code: ErrorCode, line: usize, column: usize) -> Self {
            Self {
                err: Box::new(ErrorImpl { code, line, column }),
            }
        }
    }

    let error_instance = TestError::new(ErrorCode::CustomError, 1, 0);
    assert_eq!(error_instance.line(), 1);
}

