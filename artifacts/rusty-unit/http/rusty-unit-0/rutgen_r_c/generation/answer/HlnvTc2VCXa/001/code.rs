// Answer 0

#[test]
fn test_is_with_status_code() {
    struct InvalidStatusCode;

    let error = Error {
        inner: ErrorKind::StatusCode(InvalidStatusCode),
    };

    assert!(error.is::<InvalidStatusCode>());
    assert!(!error.is::<method::InvalidMethod>());
}

#[test]
fn test_is_with_method() {
    struct InvalidMethod;

    let error = Error {
        inner: ErrorKind::Method(InvalidMethod),
    };

    assert!(error.is::<InvalidMethod>());
    assert!(!error.is::<uri::InvalidUri>());
}

#[test]
fn test_is_with_uri() {
    struct InvalidUri;

    let error = Error {
        inner: ErrorKind::Uri(InvalidUri),
    };

    assert!(error.is::<InvalidUri>());
    assert!(!error.is::<header::InvalidHeaderName>());
}

#[test]
fn test_is_with_uri_parts() {
    struct InvalidUriParts;

    let error = Error {
        inner: ErrorKind::UriParts(InvalidUriParts),
    };

    assert!(error.is::<InvalidUriParts>());
    assert!(!error.is::<header::InvalidHeaderValue>());
}

#[test]
fn test_is_with_header_name() {
    struct InvalidHeaderName;

    let error = Error {
        inner: ErrorKind::HeaderName(InvalidHeaderName),
    };

    assert!(error.is::<InvalidHeaderName>());
    assert!(!error.is::<MaxSizeReached>());
}

#[test]
fn test_is_with_header_value() {
    struct InvalidHeaderValue;

    let error = Error {
        inner: ErrorKind::HeaderValue(InvalidHeaderValue),
    };

    assert!(error.is::<InvalidHeaderValue>());
    assert!(!error.is::<ErrorKind::InvalidScheme>());
}

#[test]
fn test_is_with_max_size_reached() {
    struct MaxSizeReached;

    let error = Error {
        inner: ErrorKind::MaxSizeReached(MaxSizeReached),
    };

    assert!(error.is::<MaxSizeReached>());
    assert!(!error.is::<ErrorKind::InvalidAuthority>());
}

