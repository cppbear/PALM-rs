// Answer 0

#[test]
fn test_error_invalid_uri_char() {
    let error = Error {
        inner: ErrorKind::InvalidUriChar,
    };
    let _ = format!("{:?}", error);
}

#[test]
fn test_error_invalid_scheme() {
    let error = Error {
        inner: ErrorKind::InvalidScheme,
    };
    let _ = format!("{:?}", error);
}

#[test]
fn test_error_invalid_authority() {
    let error = Error {
        inner: ErrorKind::InvalidAuthority,
    };
    let _ = format!("{:?}", error);
}

#[test]
fn test_error_invalid_port() {
    let error = Error {
        inner: ErrorKind::InvalidPort,
    };
    let _ = format!("{:?}", error);
}

#[test]
fn test_error_invalid_format() {
    let error = Error {
        inner: ErrorKind::InvalidFormat,
    };
    let _ = format!("{:?}", error);
}

#[test]
fn test_error_scheme_missing() {
    let error = Error {
        inner: ErrorKind::SchemeMissing,
    };
    let _ = format!("{:?}", error);
}

#[test]
fn test_error_authority_missing() {
    let error = Error {
        inner: ErrorKind::AuthorityMissing,
    };
    let _ = format!("{:?}", error);
}

#[test]
fn test_error_path_and_query_missing() {
    let error = Error {
        inner: ErrorKind::PathAndQueryMissing,
    };
    let _ = format!("{:?}", error);
}

#[test]
fn test_error_too_long() {
    let error = Error {
        inner: ErrorKind::TooLong,
    };
    let _ = format!("{:?}", error);
}

#[test]
fn test_error_empty() {
    let error = Error {
        inner: ErrorKind::Empty,
    };
    let _ = format!("{:?}", error);
}

