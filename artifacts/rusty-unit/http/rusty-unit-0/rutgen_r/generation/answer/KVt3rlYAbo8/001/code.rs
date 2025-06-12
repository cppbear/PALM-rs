// Answer 0

#[test]
fn test_trace_valid_uri() {
    let valid_uri = "https://www.rust-lang.org/";
    let request = trace(valid_uri).body(()).unwrap();
    assert_eq!(request.method(), Method::TRACE);
    assert_eq!(request.uri().to_string(), valid_uri);
}

#[test]
#[should_panic]
fn test_trace_invalid_uri() {
    let invalid_uri = "invalid_uri";
    let _request = trace(invalid_uri).body(()).unwrap(); // this should panic
}

#[test]
fn test_trace_empty_uri() {
    let empty_uri = "";
    let request = trace(empty_uri).body(()).unwrap();
    assert_eq!(request.method(), Method::TRACE);
    assert_eq!(request.uri().to_string(), empty_uri);
}

#[test]
fn test_trace_relative_uri() {
    let relative_uri = "/path/to/resource";
    let request = trace(relative_uri).body(()).unwrap();
    assert_eq!(request.method(), Method::TRACE);
    assert_eq!(request.uri().to_string(), relative_uri);
}

