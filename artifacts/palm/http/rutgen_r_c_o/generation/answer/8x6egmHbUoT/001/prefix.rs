// Answer 0

#[test]
fn test_and_then_with_ok_result() {
    let parts = Parts {
        method: Method::GET,
        uri: Uri::from_static("http://example.com"),
        version: Version::HTTP_11,
        headers: HeaderMap::new(),
        extensions: Extensions::new(),
        _priv: (),
    };

    let builder = Builder {
        inner: Ok(parts),
    };

    let func = |p: Parts| Ok(p);
    let _ = builder.and_then(func);
}

#[test]
fn test_and_then_with_err_result() {
    let parts = Parts {
        method: Method::POST,
        uri: Uri::from_static("http://example.org"),
        version: Version::HTTP_10,
        headers: HeaderMap::new(),
        extensions: Extensions::new(),
        _priv: (),
    };

    let builder = Builder {
        inner: Ok(parts),
    };

    let func = |_: Parts| Err(Error::new(ErrorKind::Other));
    let _ = builder.and_then(func);
}

#[test]
fn test_and_then_with_empty_headers() {
    let parts = Parts {
        method: Method::PUT,
        uri: Uri::from_static("http://empty-headers.com"),
        version: Version::HTTP_11,
        headers: HeaderMap::new(),
        extensions: Extensions::new(),
        _priv: (),
    };

    let builder = Builder {
        inner: Ok(parts),
    };

    let func = |p: Parts| Ok(p);
    let _ = builder.and_then(func);
}

#[test]
fn test_and_then_with_complex_uri() {
    let uri = Uri::from_static("http://example.com/path?query=1");
    let parts = Parts {
        method: Method::DELETE,
        uri,
        version: Version::HTTP_11,
        headers: HeaderMap::new(),
        extensions: Extensions::new(),
        _priv: (),
    };

    let builder = Builder {
        inner: Ok(parts),
    };

    let func = |p: Parts| Ok(p);
    let _ = builder.and_then(func);
}

#[test]
fn test_and_then_with_invalid_uri() {
    let parts = Parts {
        method: Method::PATCH,
        uri: Uri::from_static("http://invalid.com"),
        version: Version::HTTP_11,
        headers: HeaderMap::with_capacity(1),
        extensions: Extensions::new(),
        _priv: (),
    };

    let builder = Builder {
        inner: Ok(parts),
    };

    let func = |_: Parts| Err(Error::new(ErrorKind::InvalidInput));
    let _ = builder.and_then(func);
}

