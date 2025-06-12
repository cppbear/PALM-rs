// Answer 0

#[test]
fn test_headers_initially_empty() {
    struct DummyResponseBody;
    let response: Response<DummyResponseBody> = Response::new(DummyResponseBody);
    assert!(response.headers().entries.is_empty());
}

#[test]
fn test_headers_non_empty() {
    struct DummyResponseBody;
    
    let mut parts = Parts {
        headers: HeaderMap {
            mask: Size::default(),
            indices: Box::new([]),
            entries: vec![Bucket::default()], // Assume at least one header for testing
            extra_values: vec![],
            danger: Danger::default(),
        },
        ..Default::default()
    };
    
    let response: Response<DummyResponseBody> = Response::from_parts(parts, DummyResponseBody);
    assert!(!response.headers().entries.is_empty());
}

