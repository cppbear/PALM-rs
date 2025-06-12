// Answer 0

#[test]
fn test_extensions_empty() {
    struct DummyBody;
    
    let response: Response<DummyBody> = Response::new(DummyBody);
    assert!(response.extensions().map.is_none());
}

#[test]
fn test_extensions_with_some_value() {
    struct DummyBody;

    let mut extensions = Extensions::default();
    extensions.map = Some(Box::new(AnyMap::new())); // Assuming AnyMap is defined in the crate

    let response_parts = Parts {
        status: StatusCode::OK,
        version: Version::HTTP_11,
        headers: HeaderMap::default(),
        extensions,
        _priv: (),
    };

    let response: Response<DummyBody> = Response::from_parts(response_parts, DummyBody);
    assert!(response.extensions().map.is_some());
}

