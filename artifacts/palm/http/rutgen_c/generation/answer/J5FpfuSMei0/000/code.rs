// Answer 0

#[test]
fn test_version_default() {
    struct DummyBody;
    let request: Request<DummyBody> = Request::new(DummyBody);
    assert_eq!(request.version(), Version::HTTP_11);
}

#[test]
fn test_version_from_parts() {
    struct DummyBody;
    let parts = Parts {
        method: Method::GET,
        uri: Uri::from_static("http://example.com"),
        version: Version::HTTP_10,
        headers: HeaderMap::new(),
        extensions: Extensions::new(),
        _priv: (),
    };
    let request: Request<DummyBody> = Request::from_parts(parts, DummyBody);
    assert_eq!(request.version(), Version::HTTP_10);
}

