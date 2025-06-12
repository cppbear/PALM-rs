// Answer 0

#[test]
fn test_trace_with_valid_uri() {
    use http::{Request, Uri};

    let uri = "https://www.rust-lang.org/";
    let request = Request::trace(uri)
        .body(())
        .unwrap();

    assert_eq!(request.method(), "TRACE");
    assert_eq!(request.uri().to_string(), uri);
}

#[test]
#[should_panic]
fn test_trace_with_invalid_uri() {
    use http::{Request, Uri};

    let uri = "invalid_uri";
    let _request = Request::trace(uri)
        .body(())
        .unwrap(); // This should panic due to invalid URI
}

