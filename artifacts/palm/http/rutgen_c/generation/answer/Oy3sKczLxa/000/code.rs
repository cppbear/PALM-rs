// Answer 0

#[test]
fn test_headers_mut_inserts_header() {
    use crate::header::{HOST, HeaderValue};
    use crate::Method;
    use crate::Uri;
    use crate::Version;

    // Create a default Parts instance
    let parts = Parts {
        method: Method::GET,
        uri: Uri::default(),
        version: Version::default(),
        headers: HeaderMap::default(),
        extensions: Extensions::default(),
        _priv: (),
    };

    // Create a new Request instance
    let mut request: Request<()> = Request::from_parts(parts, ());

    // Insert a header
    request.headers_mut().insert(HOST, HeaderValue::from_static("world"));

    // Assert the header is successfully inserted
    assert!(!request.headers().is_empty());
    assert_eq!(request.headers().get(HOST).unwrap().to_str().unwrap(), "world");
}

#[test]
fn test_headers_mut_return_type() {
    use crate::header::{HeaderValue};
    use crate::Method;
    use crate::Uri;
    use crate::Version;

    // Create a default Parts instance
    let parts = Parts {
        method: Method::GET,
        uri: Uri::default(),
        version: Version::default(),
        headers: HeaderMap::default(),
        extensions: Extensions::default(),
        _priv: (),
    };

    // Create a new Request instance
    let mut request: Request<()> = Request::from_parts(parts, ());

    // Get mutable reference to headers
    let headers_mut = request.headers_mut();

    // Check if the mutable reference is valid
    assert!(headers_mut.is_empty());
}

#[test]
fn test_headers_mut_with_empty_header_map() {
    use crate::Method;
    use crate::Uri;
    use crate::Version;

    // Create an empty Parts instance
    let parts = Parts {
        method: Method::GET,
        uri: Uri::default(),
        version: Version::default(),
        headers: HeaderMap::default(),
        extensions: Extensions::default(),
        _priv: (),
    };

    // Create a new Request instance
    let mut request: Request<()> = Request::from_parts(parts, ());

    // Mutable reference to the header map
    let headers_mut = request.headers_mut();

    // Insert a value
    headers_mut.insert(HeaderName::from_static("test"), HeaderValue::from_static("value"));

    // Assert that headers are not empty after insertion
    assert!(!request.headers().is_empty());
}

