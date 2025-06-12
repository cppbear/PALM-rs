// Answer 0

#[test]
fn test_headers_ref_success() {
    use http::{Response, header::{HeaderValue, HeaderMap}};
    
    // Constructing a response builder and adding headers
    let res = Response::builder()
        .header("Accept", "text/html")
        .header("X-Custom-Foo", "bar")
        .body(())
        .unwrap();

    // Calling the method to be tested
    let headers = res.headers_ref().unwrap();
    
    // Asserting the expected values
    assert_eq!(headers["Accept"], HeaderValue::from_static("text/html"));
    assert_eq!(headers["X-Custom-Foo"], HeaderValue::from_static("bar"));
}

#[test]
fn test_headers_ref_failure() {
    use http::Response;
    
    // Creating a response without proper body
    let res: Response<()> = Response::builder().body(()).unwrap();

    // Calling the method to be tested and expecting None
    assert!(res.headers_ref().is_none());
}

