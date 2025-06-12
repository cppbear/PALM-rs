// Answer 0

#[test]
fn test_request_from_parts_valid() {
    use crate::header::{HeaderMap, HeaderValue};
    use crate::method::Method;
    use crate::version::Version;
    use crate::Uri;
    use crate::Extensions;

    let mut headers = HeaderMap::new();
    headers.insert(HeaderName::from_static("content-type"), HeaderValue::from_static("text/plain"));

    let parts = Parts {
        method: Method::GET,
        uri: Uri::from_static("http://example.com"),
        version: Version::HTTP_11,
        headers,
        extensions: Extensions::new(),
        _priv: (),
    };

    let body = "Hello, world!";
    
    let request = Request::from_parts(parts.clone(), body);
    
    assert_eq!(request.method(), &parts.method);
    assert_eq!(request.uri(), &parts.uri);
    assert_eq!(request.version(), parts.version);
    assert_eq!(request.headers(), &parts.headers);
    assert_eq!(request.extensions(), &parts.extensions);
    assert_eq!(request.body(), &body);
}

#[test]
fn test_request_from_parts_with_empty_body() {
    use crate::header::{HeaderMap, HeaderValue};
    use crate::method::Method;
    use crate::version::Version;
    use crate::Uri;
    use crate::Extensions;

    let mut headers = HeaderMap::new();
    headers.insert(HeaderName::from_static("content-type"), HeaderValue::from_static("application/json"));

    let parts = Parts {
        method: Method::POST,
        uri: Uri::from_static("http://example.com"),
        version: Version::HTTP_11,
        headers,
        extensions: Extensions::new(),
        _priv: (),
    };

    let body = "";

    let request = Request::from_parts(parts.clone(), body);
    
    assert_eq!(request.method(), &parts.method);
    assert_eq!(request.uri(), &parts.uri);
    assert_eq!(request.version(), parts.version);
    assert_eq!(request.headers(), &parts.headers);
    assert_eq!(request.extensions(), &parts.extensions);
    assert_eq!(request.body(), &body);
}

