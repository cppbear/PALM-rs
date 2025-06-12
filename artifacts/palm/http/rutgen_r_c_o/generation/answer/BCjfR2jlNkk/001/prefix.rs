// Answer 0

#[test]
fn test_extensions_empty() {
    let body = ();
    let extensions = Extensions::default();
    let headers = HeaderMap::new();
    let parts = Parts {
        status: StatusCode::OK,
        version: Version::HTTP_11,
        headers,
        extensions,
        _priv: (),
    };
    let response = Response::from_parts(parts, body);
    let _ = response.extensions();
}

#[test]
fn test_extensions_with_data() {
    let body = "Response body";
    let mut extensions = Extensions::default();
    extensions.map = Some(Box::new(AnyMap::new()));
    let headers = HeaderMap::new();
    let parts = Parts {
        status: StatusCode::CREATED,
        version: Version::HTTP_2,
        headers,
        extensions,
        _priv: (),
    };
    let response = Response::from_parts(parts, body);
    let _ = response.extensions();
}

#[test]
fn test_extensions_large_body() {
    let body = vec![0u8; 1_000_000];
    let mut extensions = Extensions::default();
    extensions.map = Some(Box::new(AnyMap::new()));
    let headers = HeaderMap::new();
    let parts = Parts {
        status: StatusCode::NO_CONTENT,
        version: Version::HTTP_11,
        headers,
        extensions,
        _priv: (),
    };
    let response = Response::from_parts(parts, body);
    let _ = response.extensions();
}

#[test]
fn test_extensions_non_empty_headers() {
    let body = "Sample body";
    let mut extensions = Extensions::default();
    extensions.map = Some(Box::new(AnyMap::new()));
    let mut headers = HeaderMap::new();
    headers.insert(HeaderName::from_static("content-type"), HeaderValue::from_static("text/plain"));
    let parts = Parts {
        status: StatusCode::BAD_REQUEST,
        version: Version::HTTP_11,
        headers,
        extensions,
        _priv: (),
    };
    let response = Response::from_parts(parts, body);
    let _ = response.extensions();
}

#[test]
#[should_panic]
fn test_extensions_with_uninitialized_map() {
    let body = "Test body";
    let headers = HeaderMap::new();
    let parts = Parts {
        status: StatusCode::FORBIDDEN,
        version: Version::HTTP_11,
        headers,
        extensions: Extensions::default(),
        _priv: (),
    };
    let response = Response::from_parts(parts, body);
    let _ = response.extensions();
}

