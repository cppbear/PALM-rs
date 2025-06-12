// Answer 0

#[test]
fn test_into_parts_empty_body() {
    struct DummyBody;

    let request = Request::new(DummyBody);
    let (parts, body) = request.into_parts();
    
    assert_eq!(parts.method, Method::GET);
    assert_eq!(parts.uri, Uri::default()); // Using default for demonstration
    assert_eq!(parts.version, Version::default()); // Assuming default
}

#[test]
fn test_into_parts_with_body() {
    struct DummyBody;

    let body = DummyBody;
    let parts = Parts {
        method: Method::POST,
        uri: Uri::default(), // Using default for demonstration
        version: Version::default(), // Assuming default
        headers: HeaderMap::new(), // Initialize with empty headers
        extensions: Extensions::new(), // Initialize with empty extensions
        _priv: (),
    };

    let request = Request::from_parts(parts.clone(), body);
    let (returned_parts, returned_body) = request.into_parts();
    
    assert_eq!(returned_parts.method, parts.method);
    assert_eq!(returned_parts.uri, parts.uri);
    assert_eq!(returned_parts.version, parts.version);
    assert_eq!(returned_body, body);
}

