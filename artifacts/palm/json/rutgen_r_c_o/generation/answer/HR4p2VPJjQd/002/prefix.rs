// Answer 0

#[test]
fn test_io_error_kind_not_found() {
    let error = Error {
        err: Box::new(ErrorImpl {
            code: ErrorCode::Io(io::Error::new(io::ErrorKind::NotFound, "not found")),
            line: 0,
            column: 0,
        }),
    };
    let _ = error.io_error_kind();
}

#[test]
fn test_io_error_kind_permission_denied() {
    let error = Error {
        err: Box::new(ErrorImpl {
            code: ErrorCode::Io(io::Error::new(io::ErrorKind::PermissionDenied, "permission denied")),
            line: 0,
            column: 0,
        }),
    };
    let _ = error.io_error_kind();
}

#[test]
fn test_io_error_kind_connection_refused() {
    let error = Error {
        err: Box::new(ErrorImpl {
            code: ErrorCode::Io(io::Error::new(io::ErrorKind::ConnectionRefused, "connection refused")),
            line: 0,
            column: 0,
        }),
    };
    let _ = error.io_error_kind();
}

#[test]
fn test_io_error_kind_timed_out() {
    let error = Error {
        err: Box::new(ErrorImpl {
            code: ErrorCode::Io(io::Error::new(io::ErrorKind::TimedOut, "timed out")),
            line: 0,
            column: 0,
        }),
    };
    let _ = error.io_error_kind();
}

#[test]
fn test_io_error_kind_unexpected_eof() {
    let error = Error {
        err: Box::new(ErrorImpl {
            code: ErrorCode::Io(io::Error::new(io::ErrorKind::UnexpectedEof, "unexpected eof")),
            line: 0,
            column: 0,
        }),
    };
    let _ = error.io_error_kind();
}

#[test]
fn test_io_error_kind_broken_pipe() {
    let error = Error {
        err: Box::new(ErrorImpl {
            code: ErrorCode::Io(io::Error::new(io::ErrorKind::BrokenPipe, "broken pipe")),
            line: 0,
            column: 0,
        }),
    };
    let _ = error.io_error_kind();
}

#[test]
fn test_io_error_kind_already_exists() {
    let error = Error {
        err: Box::new(ErrorImpl {
            code: ErrorCode::Io(io::Error::new(io::ErrorKind::AlreadyExists, "already exists")),
            line: 0,
            column: 0,
        }),
    };
    let _ = error.io_error_kind();
}

#[test]
fn test_io_error_kind_would_block() {
    let error = Error {
        err: Box::new(ErrorImpl {
            code: ErrorCode::Io(io::Error::new(io::ErrorKind::WouldBlock, "would block")),
            line: 0,
            column: 0,
        }),
    };
    let _ = error.io_error_kind();
}

