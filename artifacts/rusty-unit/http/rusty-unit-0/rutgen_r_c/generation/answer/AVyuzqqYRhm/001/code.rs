// Answer 0

#[test]
fn test_headers_mut_insert() {
    use crate::header::{HeaderValue, HOST};
    use crate::{Response, Parts, HeaderMap};

    // Create a default Parts instance
    let parts = Parts {
        status: StatusCode::OK,
        version: Version::HTTP_11,
        headers: HeaderMap::default(),
        extensions: Extensions::new(),
        _priv: (),
    };

    // Create a new Response instance with empty body
    let mut response: Response<()> = Response::from_parts(parts, ());

    // Modify headers
    response.headers_mut().insert(HOST, HeaderValue::from_static("world"));
    
    // Assert that the headers are not empty after insertion
    assert!(!response.headers().is_empty());
    assert_eq!(response.headers().get(HOST).unwrap().to_str().unwrap(), "world");
}

#[test]
fn test_headers_mut_no_panic_on_empty() {
    use crate::{Response, Parts};

    // Create a default Parts instance
    let parts = Parts {
        status: StatusCode::OK,
        version: Version::HTTP_11,
        headers: HeaderMap::default(),
        extensions: Extensions::new(),
        _priv: (),
    };

    // Create a new Response instance with empty body
    let mut response: Response<()> = Response::from_parts(parts, ());

    // Grab mutable reference to headers without inserting
    let headers = response.headers_mut();

    // Assert that headers can be accessed and mutated without any panic
    assert!(headers.is_empty());
}

