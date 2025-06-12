// Answer 0

#[test]
fn test_extensions_default() {
    struct EmptyBody;

    let response: Response<EmptyBody> = Response::new(EmptyBody);
    assert!(response.extensions().map.is_none());
}

#[test]
fn test_extensions_with_some_value() {
    struct TestBody;

    let mut extensions = Extensions::default();
    extensions.map = Some(Box::new(AnyMap::new()));

    let mut parts = Parts::default();
    parts.extensions = extensions;

    let response: Response<TestBody> = Response::from_parts(parts, TestBody);
    assert!(response.extensions().map.is_some());
}

