// Answer 0

#[test]
fn test_method_get() {
    let method = Method("GET".into());
    let headers = HeaderMap::new();
    let extensions = Extensions::default();
    let parts = Parts {
        method,
        uri: Uri::default(),
        version: Version::default(),
        headers,
        extensions,
        _priv: (),
    };
    let body = ();
    let request = Request::from_parts(parts, body);
    let _ = request.method();
}

#[test]
fn test_method_post() {
    let method = Method("POST".into());
    let headers = HeaderMap::new();
    let extensions = Extensions::default();
    let parts = Parts {
        method,
        uri: Uri::default(),
        version: Version::default(),
        headers,
        extensions,
        _priv: (),
    };
    let body = ();
    let request = Request::from_parts(parts, body);
    let _ = request.method();
}

#[test]
fn test_method_with_non_empty_headers() {
    let method = Method("PUT".into());
    let mut headers = HeaderMap::new();
    headers.insert(HeaderName::from_static("content-type"), HeaderValue::from_static("application/json"));
    let extensions = Extensions::default();
    let parts = Parts {
        method,
        uri: Uri::default(),
        version: Version::default(),
        headers,
        extensions,
        _priv: (),
    };
    let body = ();
    let request = Request::from_parts(parts, body);
    let _ = request.method();
}

#[test]
fn test_method_with_non_empty_extensions() {
    let method = Method("DELETE".into());
    let headers = HeaderMap::new();
    let mut extensions = Extensions::default();
    extensions.insert("key".into(), "value".into());
    let parts = Parts {
        method,
        uri: Uri::default(),
        version: Version::default(),
        headers,
        extensions,
        _priv: (),
    };
    let body = ();
    let request = Request::from_parts(parts, body);
    let _ = request.method();
}

