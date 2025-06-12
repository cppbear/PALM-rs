// Answer 0

#[test]
fn test_response_from_parts_with_valid_data() {
    struct TestBody;

    let parts = Parts {
        status: StatusCode::OK,
        version: Version::default(),
        headers: HeaderMap::new(),
        extensions: Extensions::new(),
        _priv: (),
    };

    let body = TestBody;
    let response = Response::from_parts(parts.clone(), body);

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.version(), Version::default());
    assert_eq!(response.headers(), &HeaderMap::new());
}

#[test]
fn test_response_from_parts_with_empty_body() {
    struct EmptyBody;

    let parts = Parts {
        status: StatusCode::CREATED,
        version: Version::default(),
        headers: HeaderMap::new(),
        extensions: Extensions::new(),
        _priv: (),
    };

    let body = EmptyBody;
    let response = Response::from_parts(parts.clone(), body);

    assert_eq!(response.status(), StatusCode::CREATED);
    assert_eq!(response.version(), Version::default());
    assert_eq!(response.headers(), &HeaderMap::new());
}

#[test]
fn test_response_from_parts_with_custom_headers() {
    struct SampleBody;

    let mut headers = HeaderMap::new();
    headers.insert(HeaderName::from_static("x-custom-header"), HeaderValue::from_static("value"));

    let parts = Parts {
        status: StatusCode::NO_CONTENT,
        version: Version::default(),
        headers,
        extensions: Extensions::new(),
        _priv: (),
    };

    let body = SampleBody;
    let response = Response::from_parts(parts.clone(), body);

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    assert_eq!(response.headers().get(HeaderName::from_static("x-custom-header")), Some(&HeaderValue::from_static("value")));
}

