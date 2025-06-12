// Answer 0

#[test]
fn test_io_error_kind_with_timed_out_error() {
    struct TestError {
        err: ErrorCode,
    }

    impl TestError {
        pub fn new(kind: ErrorKind) -> Self {
            TestError {
                err: ErrorCode::Io(io::Error::new(kind, "test error")),
            }
        }
    }

    let error = TestError::new(ErrorKind::TimedOut);
    assert_eq!(error.io_error_kind(), Some(ErrorKind::TimedOut));
}

#[test]
fn test_io_error_kind_with_other_error() {
    struct TestError {
        err: ErrorCode,
    }

    impl TestError {
        pub fn new(kind: ErrorKind) -> Self {
            TestError {
                err: ErrorCode::Io(io::Error::new(kind, "test error")),
            }
        }
    }

    let error = TestError::new(ErrorKind::NotFound);
    assert_eq!(error.io_error_kind(), Some(ErrorKind::NotFound));
}

#[test]
fn test_io_error_kind_with_non_io_error() {
    struct TestError {
        err: ErrorCode,
    }

    impl TestError {
        pub fn new() -> Self {
            TestError { err: ErrorCode::Other }
        }
    }

    let error = TestError::new();
    assert_eq!(error.io_error_kind(), None);
}

