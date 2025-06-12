// Answer 0

#[test]
fn test_into_parts_with_empty_request_body() {
    struct DummyBody;

    let request = Request::new(DummyBody);
    let (parts, body) = request.into_parts();
    
    assert_eq!(parts.method, Method::GET);
    assert_eq!(body, DummyBody);
}

#[test]
fn test_into_parts_with_non_empty_request_body() {
    struct NonEmptyBody {
        content: String,
    }

    let request_body = NonEmptyBody {
        content: String::from("Hello, World!"),
    };
    let request = Request::new(request_body);
    let (parts, body) = request.into_parts();
    
    assert_eq!(parts.method, Method::GET);
    assert_eq!(body.content, "Hello, World!");
}

#[test]
fn test_into_parts_with_different_method() {
    struct DummyBody;

    let mut parts = Parts {
        method: Method::POST,
        uri: Uri::default(),
        version: Version::default(),
        headers: HeaderMap::new(),
        extensions: Extensions::new(),
        _priv: (),
    };
    let request = Request::from_parts(parts.clone(), DummyBody);
    let (returned_parts, body) = request.into_parts();
    
    assert_eq!(returned_parts.method, Method::POST);
    assert_eq!(body, DummyBody);
}

#[test]
fn test_into_parts_with_headers() {
    struct DummyBody;

    let mut headers = HeaderMap::new();
    headers.insert(HeaderName::from_static("content-type"), HeaderValue::from_static("application/json"));

    let request = Request::from_parts(
        Parts {
            method: Method::GET,
            uri: Uri::default(),
            version: Version::default(),
            headers,
            extensions: Extensions::new(),
            _priv: (),
        },
        DummyBody,
    );

    let (parts, body) = request.into_parts();
    assert_eq!(parts.method, Method::GET);
    assert_eq!(parts.headers.get(HeaderName::from_static("content-type")).unwrap(), &HeaderValue::from_static("application/json"));
    assert_eq!(body, DummyBody);
}

