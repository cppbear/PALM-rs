// Answer 0

#[test]
fn test_is_data_correct_category() {
    struct TestError {
        err: Box<ErrorImpl>,
    }

    impl Error {
        fn new(code: ErrorCode, line: usize, column: usize) -> Self {
            let error_impl = ErrorImpl { code, line, column };
            Error {
                err: Box::new(error_impl),
            }
        }
    }

    let error = TestError::new(ErrorCode::Message("Invalid data".to_string()), 1, 1);
    assert!(error.is_data());
}

#[test]
fn test_is_data_incorrect_category() {
    struct TestError {
        err: Box<ErrorImpl>,
    }

    impl Error {
        fn new(code: ErrorCode, line: usize, column: usize) -> Self {
            let error_impl = ErrorImpl { code, line, column };
            Error {
                err: Box::new(error_impl),
            }
        }
    }

    let error = TestError::new(ErrorCode::Io("Unable to read".to_string()), 1, 1);
    assert!(!error.is_data());
}

