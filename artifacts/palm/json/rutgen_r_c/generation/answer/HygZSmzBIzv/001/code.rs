// Answer 0

#[test]
fn test_io_error() {
    use std::io;

    let std_error = io::Error::new(io::ErrorKind::Other, "test error");
    let error = Error::io(std_error);

    assert_eq!(error.err.line, 0);
    assert_eq!(error.err.column, 0);
    if let ErrorCode::Io(ref e) = *error.err.code {
        assert_eq!(e.kind(), io::ErrorKind::Other);
        assert_eq!(e.to_string(), "test error");
    } else {
        panic!("Expected ErrorCode::Io");
    }
}

#[test]
fn test_io_error_kind_not_found() {
    use std::io;

    let std_error = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let error = Error::io(std_error);

    assert_eq!(error.err.line, 0);
    assert_eq!(error.err.column, 0);
    if let ErrorCode::Io(ref e) = *error.err.code {
        assert_eq!(e.kind(), io::ErrorKind::NotFound);
        assert_eq!(e.to_string(), "file not found");
    } else {
        panic!("Expected ErrorCode::Io");
    }
}

