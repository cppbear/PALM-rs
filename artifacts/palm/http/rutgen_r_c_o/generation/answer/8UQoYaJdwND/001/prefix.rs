// Answer 0

#[test]
fn test_extensions_empty() {
    let body = ();
    let parts = Parts {
        method: Method::GET,
        uri: "http://example.com".parse().unwrap(),
        version: Version::HTTP_11,
        headers: HeaderMap::new(),
        extensions: Extensions::default(),
        _priv: (),
    };
    let request = Request::from_parts(parts, body);
    request.extensions();
}

#[test]
fn test_extensions_with_anymap() {
    let body = ();
    let mut extensions = Extensions::default();
    extensions.map = Some(Box::new(AnyMap::new()));
    
    let parts = Parts {
        method: Method::POST,
        uri: "http://example.com".parse().unwrap(),
        version: Version::HTTP_11,
        headers: HeaderMap::new(),
        extensions,
        _priv: (),
    };
    let request = Request::from_parts(parts, body);
    request.extensions();
}

#[test]
fn test_extensions_non_empty() {
    let body = ();
    let mut extensions = Extensions::default();
    let mut anymap = AnyMap::new();
    anymap.insert(42i32);
    extensions.map = Some(Box::new(anymap));
    
    let parts = Parts {
        method: Method::PUT,
        uri: "http://example.com/resource".parse().unwrap(),
        version: Version::HTTP_11,
        headers: HeaderMap::new(),
        extensions,
        _priv: (),
    };
    let request = Request::from_parts(parts, body);
    request.extensions();
}

#[test]
fn test_extensions_with_custom_body() {
    struct CustomBody {
        data: String,
    }
    
    let body = CustomBody { data: "test".into() };
    let mut extensions = Extensions::default();
    extensions.map = Some(Box::new(AnyMap::new()));
    
    let parts = Parts {
        method: Method::HEAD,
        uri: "http://example.com/custom".parse().unwrap(),
        version: Version::HTTP_11,
        headers: HeaderMap::new(),
        extensions,
        _priv: (),
    };
    let request = Request::from_parts(parts, body);
    request.extensions();
}

#[test]
fn test_extensions_with_different_body_type() {
    struct AnotherBody {
        count: i32,
    }
    
    let body = AnotherBody { count: 10 };
    
    let parts = Parts {
        method: Method::OPTIONS,
        uri: "http://example.com/another".parse().unwrap(),
        version: Version::HTTP_11,
        headers: HeaderMap::new(),
        extensions: Extensions::default(),
        _priv: (),
    };
    let request = Request::from_parts(parts, body);
    request.extensions();
}

