// Answer 0

#[test]
fn test_io_error_kind_with_io_error() {
    struct ErrorBox {
        code: ErrorCode,
        line: usize,
        column: usize,
    }

    let io_error = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let error_impl = ErrorImpl {
        code: ErrorCode::Io(io_error),
        line: 1,
        column: 5,
    };
    let error = Error {
        err: Box::new(error_impl),
    };
    
    assert_eq!(error.io_error_kind(), Some(io::ErrorKind::NotFound));
}

#[test]
fn test_io_error_kind_with_non_io_error() {
    struct ErrorBox {
        code: ErrorCode,
        line: usize,
        column: usize,
    }

    let error_impl = ErrorImpl {
        code: ErrorCode::Message(Box::<str>::from("syntax error")),
        line: 2,
        column: 10,
    };
    let error = Error {
        err: Box::new(error_impl),
    };
    
    assert_eq!(error.io_error_kind(), None);
}

