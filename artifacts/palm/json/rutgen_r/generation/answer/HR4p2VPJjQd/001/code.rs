// Answer 0

#[test]
fn test_io_error_kind_not_io_error() {
    struct TestError {
        err: ErrorCode,
    }

    enum ErrorCode {
        Io(std::io::Error),
        Other,
    }

    impl TestError {
        pub fn io_error_kind(&self) -> Option<std::io::ErrorKind> {
            if let ErrorCode::Io(io_error) = &self.err {
                Some(io_error.kind())
            } else {
                None
            }
        }
    }

    let error = TestError { err: ErrorCode::Other };
    assert_eq!(error.io_error_kind(), None);
}

#[test]
fn test_io_error_kind_no_io_error() {
    struct TestError {
        err: ErrorCode,
    }

    enum ErrorCode {
        Io(std::io::Error),
        Other,
    }

    impl TestError {
        pub fn io_error_kind(&self) -> Option<std::io::ErrorKind> {
            if let ErrorCode::Io(io_error) = &self.err {
                Some(io_error.kind())
            } else {
                None
            }
        }
    }

    let error = TestError { err: ErrorCode::Other };
    assert_eq!(error.io_error_kind(), None);
}

