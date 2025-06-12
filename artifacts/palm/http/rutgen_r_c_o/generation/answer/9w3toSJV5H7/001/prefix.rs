// Answer 0

#[test]
fn test_into_parts_with_empty_body() {
    let method = Method::GET;
    let uri = Uri::from_static("http://example.com");
    let version = Version::HTTP_1_1;
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
    
    let request = Request::from_parts(parts, ());
    let (head, body) = request.into_parts();
}

#[test]
fn test_into_parts_with_string_body() {
    let method = Method::GET;
    let uri = Uri::from_static("http://example.com");
    let version = Version::HTTP_1_1;
    let mut headers = HeaderMap::new();
    headers.insert(HeaderName::from_static("content-type"), HeaderValue::from_static("application/json"));
    let extensions = Extensions::new();

    let parts = Parts {
        method,
        uri,
        version,
        headers,
        extensions,
        _priv: (),
    };
    
    let request = Request::from_parts(parts, String::from("This is a body"));
    let (head, body) = request.into_parts();
}

#[test]
fn test_into_parts_with_struct_body() {
    struct CustomBody {
        content: i32,
    }
    
    let method = Method::GET;
    let uri = Uri::from_static("http://example.com");
    let version = Version::HTTP_1_1;
    let mut headers = HeaderMap::new();
    headers.insert(HeaderName::from_static("custom-header"), HeaderValue::from_static("custom-value"));
    let extensions = Extensions::new();

    let parts = Parts {
        method,
        uri,
        version,
        headers,
        extensions,
        _priv: (),
    };

    let request = Request::from_parts(parts, CustomBody { content: 42 });
    let (head, body) = request.into_parts();
}

#[test]
fn test_into_parts_with_non_empty_headers() {
    let method = Method::GET;
    let uri = Uri::from_static("http://example.com");
    let version = Version::HTTP_1_1;
    let mut headers = HeaderMap::new();
    headers.insert(HeaderName::from_static("accept"), HeaderValue::from_static("text/html"));
    headers.insert(HeaderName::from_static("user-agent"), HeaderValue::from_static("my-client"));
    let extensions = Extensions::new();

    let parts = Parts {
        method,
        uri,
        version,
        headers,
        extensions,
        _priv: (),
    };

    let request = Request::from_parts(parts, vec![1, 2, 3]);
    let (head, body) = request.into_parts();
}

