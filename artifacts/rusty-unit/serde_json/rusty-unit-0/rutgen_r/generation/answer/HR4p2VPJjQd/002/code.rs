// Answer 0

#[test]
fn test_io_error_kind_with_io_error() {
    struct MockError {
        code: ErrorCode,
    }

    struct ErrorCode {
        kind: ErrorKind,
    }

    struct ErrorWrapper {
        err: MockError,
    }

    impl ErrorWrapper {
        pub fn io_error_kind(&self) -> Option<ErrorKind> {
            if let ErrorCode::Io(io_error) = &self.err.code {
                Some(io_error.kind())
            } else {
                None
            }
        }
    }

    let io_error = ErrorCode { kind: ErrorKind::PermissionDenied };
    let mock_error = MockError { code: ErrorCode::Io(io_error) };
    let error_wrapper = ErrorWrapper { err: mock_error };

    assert_eq!(error_wrapper.io_error_kind(), Some(ErrorKind::PermissionDenied));
}

#[test]
fn test_io_error_kind_with_non_io_error() {
    struct MockError {
        code: ErrorCode,
    }

    enum ErrorCode {
        Other,
    }

    struct ErrorWrapper {
        err: MockError,
    }

    impl ErrorWrapper {
        pub fn io_error_kind(&self) -> Option<ErrorKind> {
            if let ErrorCode::Io(io_error) = &self.err.code {
                Some(io_error.kind())
            } else {
                None
            }
        }
    }

    let mock_error = MockError { code: ErrorCode::Other };
    let error_wrapper = ErrorWrapper { err: mock_error };

    assert_eq!(error_wrapper.io_error_kind(), None);
}

