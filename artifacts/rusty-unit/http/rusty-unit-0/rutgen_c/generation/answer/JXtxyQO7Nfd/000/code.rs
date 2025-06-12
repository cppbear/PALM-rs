// Answer 0

#[test]
fn test_into_parts() {
    struct DummyBody;
    
    let parts = Parts {
        status: StatusCode::OK,
        version: Version::default(),
        headers: HeaderMap::new(),
        extensions: Extensions::new(),
        _priv: (),
    };
    
    let response: Response<DummyBody> = Response::from_parts(parts.clone(), DummyBody);
    let (returned_parts, body) = response.into_parts();
    
    assert_eq!(returned_parts.status, StatusCode::OK);
    assert_eq!(returned_parts.version, Version::default());
}

#[test]
fn test_into_parts_with_default_response() {
    struct DummyBody;

    let response: Response<DummyBody> = Response::default();
    let (parts, body) = response.into_parts();
    
    assert_eq!(parts.status, StatusCode::OK);
    assert_eq!(parts.version, Version::default());
}

#[test]
#[should_panic]
fn test_into_parts_with_uninitialized_response() {
    struct DummyBody;

    let response: Response<DummyBody> = Response::new(DummyBody);
    let (parts, body) = response.into_parts();
    assert!(parts.headers.is_empty());  // Just to force a panic on uninitialized
}

