// Answer 0

#[test]
fn test_map_with_string_body() {
    struct TestRequest {
        method: Method,
        uri: Uri,
        version: Version,
        headers: HeaderMap<HeaderValue>,
        extensions: Extensions,
    }

    let body = String::from("some string");
    let method = Method::GET;
    let uri = Uri::from_static("http://example.com");
    let version = Version::HTTP_11;
    let headers = HeaderMap::new();
    let extensions = Extensions::new();
    let parts = Parts {
        method,
        uri,
        version,
        headers,
        extensions,
        _priv: (),
    };
    let request: Request<String> = Request::from_parts(parts, body);

    let mapped_request: Request<&[u8]> = request.map(|b| {
        assert_eq!(b, "some string");
        b.as_bytes()
    });
    
    assert_eq!(mapped_request.body(), &"some string".as_bytes());
}

#[test]
fn test_map_with_integer_body() {
    let body = 42;
    let method = Method::POST;
    let uri = Uri::from_static("http://example.com");
    let version = Version::HTTP_11;
    let headers = HeaderMap::new();
    let extensions = Extensions::new();
    let parts = Parts {
        method,
        uri,
        version,
        headers,
        extensions,
        _priv: (),
    };

    let request: Request<i32> = Request::from_parts(parts, body);

    let mapped_request: Request<String> = request.map(|b| {
        assert_eq!(b, 42);
        b.to_string()
    });

    assert_eq!(mapped_request.body(), "42");
}

#[test]
fn test_map_with_empty_body() {
    let body = String::new();
    let method = Method::PUT;
    let uri = Uri::from_static("http://example.com");
    let version = Version::HTTP_11;
    let headers = HeaderMap::new();
    let extensions = Extensions::new();
    let parts = Parts {
        method,
        uri,
        version,
        headers,
        extensions,
        _priv: (),
    };

    let request: Request<String> = Request::from_parts(parts, body);

    let mapped_request: Request<&str> = request.map(|b| {
        assert_eq!(b, "");
        b.as_str()
    });

    assert_eq!(mapped_request.body(), "");
}

