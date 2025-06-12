// Answer 0

#[test]
fn test_headers_ref_with_valid_headers() {
    use http::{Response, header::HeaderValue, header::HeaderMap};
    
    let mut response_builder = Response::builder();
    response_builder = response_builder.header("Accept", "text/html");
    response_builder = response_builder.header("X-Custom-Foo", "bar");

    let response = response_builder.body(()).unwrap();
    let headers = response.headers_ref().unwrap();

    assert_eq!(headers["Accept"], HeaderValue::from_static("text/html"));
    assert_eq!(headers["X-Custom-Foo"], HeaderValue::from_static("bar"));
}

#[test]
fn test_headers_ref_with_empty_headers() {
    use http::Response;

    let response = Response::builder().body(()).unwrap();
    let headers = response.headers_ref();

    assert!(headers.is_none());
}

#[test]
#[should_panic]
fn test_headers_ref_with_error_in_builder() {
    use http::Response;

    let response_builder = Response::builder()
        .status(500); // Assume this introduces an error in building
    let response = response_builder.body(()).unwrap(); // This would be valid, but we assume that it may lead to an error in real scenarios

    // Manually triggering the error
    let _ = response.headers_ref().unwrap(); // This should panic if headers aren't valid
} 

#[test]
fn test_headers_ref_with_single_header() {
    use http::{Response, header::HeaderValue};

    let response = Response::builder()
        .header("User-Agent", "Rust")
        .body(())
        .unwrap();

    let headers = response.headers_ref().unwrap();

    assert_eq!(headers["User-Agent"], HeaderValue::from_static("Rust"));
} 

#[test]
fn test_headers_ref_with_varied_header_values() {
    use http::{Response, header::HeaderValue};

    let response = Response::builder()
        .header("Content-Type", "application/json")
        .header("Cache-Control", "no-cache")
        .body(())
        .unwrap();

    let headers = response.headers_ref().unwrap();

    assert_eq!(headers["Content-Type"], HeaderValue::from_static("application/json"));
    assert_eq!(headers["Cache-Control"], HeaderValue::from_static("no-cache"));
}

