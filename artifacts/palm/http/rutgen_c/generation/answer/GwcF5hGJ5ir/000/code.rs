// Answer 0

#[test]
fn test_request_from_parts_creates_request_correctly() {
    struct DummyBody;
    
    let parts = Parts {
        method: Method::GET,
        uri: Uri::from_static("http://example.com"),
        version: Version::new(1, 1),
        headers: HeaderMap::new(),
        extensions: Extensions::new(),
        _priv: (),
    };
    let body = DummyBody;
    
    let request = Request::from_parts(parts.clone(), body);
    
    assert_eq!(request.method(), &parts.method);
    assert_eq!(request.uri(), &parts.uri);
    assert_eq!(request.version(), parts.version);
    assert_eq!(request.headers(), &parts.headers);
    assert_eq!(request.extensions(), &parts.extensions);
}

#[test]
fn test_request_from_parts_with_different_body() {
    struct BodyA;
    struct BodyB;

    let parts = Parts {
        method: Method::POST,
        uri: Uri::from_static("http://example.com/resource"),
        version: Version::new(1, 1),
        headers: HeaderMap::new(),
        extensions: Extensions::new(),
        _priv: (),
    };
    
    let body_a = BodyA;
    let request_a = Request::from_parts(parts.clone(), body_a);
    
    let body_b = BodyB;
    let request_b = Request::from_parts(parts.clone(), body_b);
    
    assert_ne!(request_a.body(), request_b.body());
}

#[test]
#[should_panic]
fn test_request_from_parts_panic_on_empty_parts() {
    struct EmptyBody;

    let parts = Parts {
        method: Method::GET,
        uri: Uri::from_static(""),
        version: Version::new(1, 1),
        headers: HeaderMap::new(),
        extensions: Extensions::new(),
        _priv: (),
    };
    
    let body = EmptyBody;
    
    let request = Request::from_parts(parts, body);
    assert!(request.uri().to_string().is_empty());
}

