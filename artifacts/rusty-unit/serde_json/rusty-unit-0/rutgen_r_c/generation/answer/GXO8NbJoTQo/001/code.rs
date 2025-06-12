// Answer 0

#[test]
fn test_is_eof_with_eof_error() {
    struct TestError {
        code: ErrorCode,
        line: usize,
        column: usize,
    }

    struct TestErrorImpl {
        err: Box<TestError>,
    }

    impl Error {
        fn new(code: ErrorCode, line: usize, column: usize) -> Self {
            Error {
                err: Box::new(TestErrorImpl {
                    err: Box::new(TestError { code, line, column }),
                }),
            }
        }
    }

    let eof_error = Error::new(ErrorCode::EofWhileParsingValue, 1, 10);
    assert_eq!(eof_error.is_eof(), true);
}

#[test]
fn test_is_eof_with_syntax_error() {
    struct TestError {
        code: ErrorCode,
        line: usize,
        column: usize,
    }

    struct TestErrorImpl {
        err: Box<TestError>,
    }

    impl Error {
        fn new(code: ErrorCode, line: usize, column: usize) -> Self {
            Error {
                err: Box::new(TestErrorImpl {
                    err: Box::new(TestError { code, line, column }),
                }),
            }
        }
    }

    let syntax_error = Error::new(ErrorCode::InvalidNumber, 2, 5);
    assert_eq!(syntax_error.is_eof(), false);
}

#[test]
fn test_is_eof_with_io_error() {
    struct TestError {
        code: ErrorCode,
        line: usize,
        column: usize,
    }

    struct TestErrorImpl {
        err: Box<TestError>,
    }

    impl Error {
        fn new(code: ErrorCode, line: usize, column: usize) -> Self {
            Error {
                err: Box::new(TestErrorImpl {
                    err: Box::new(TestError { code, line, column }),
                }),
            }
        }
    }

    let io_error = Error::new(ErrorCode::Io(ErrorKind::NotFound), 3, 12);
    assert_eq!(io_error.is_eof(), false);
}

#[test]
fn test_is_eof_with_data_error() {
    struct TestError {
        code: ErrorCode,
        line: usize,
        column: usize,
    }

    struct TestErrorImpl {
        err: Box<TestError>,
    }

    impl Error {
        fn new(code: ErrorCode, line: usize, column: usize) -> Self {
            Error {
                err: Box::new(TestErrorImpl {
                    err: Box::new(TestError { code, line, column }),
                }),
            }
        }
    }

    let data_error = Error::new(ErrorCode::Message("data error".to_string()), 4, 8);
    assert_eq!(data_error.is_eof(), false);
}

