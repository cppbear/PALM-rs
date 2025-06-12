// Answer 0

#[test]
fn test_extensions_empty() {
    struct DummyBody;
    let request: Request<DummyBody> = Request::new(DummyBody {});
    assert!(request.extensions().map.is_none());
}

#[test]
fn test_extensions_with_anymap() {
    struct DummyBody;
    let mut extensions = Extensions::default();
    extensions.map = Some(Box::new(AnyMap::new()));

    let parts = Parts {
        extensions,
        ..Default::default()
    };

    let request: Request<DummyBody> = Request::from_parts(parts, DummyBody {});
    assert!(request.extensions().map.is_some());
}

