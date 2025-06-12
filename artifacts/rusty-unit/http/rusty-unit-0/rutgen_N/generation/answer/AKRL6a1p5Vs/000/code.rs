// Answer 0

#[test]
fn test_header_valid_key_value() {
    use http::{Request, header::HeaderValue};

    let req = Request::builder()
        .header("Accept", "text/html")
        .header("X-Custom-Foo", "bar")
        .body(())
        .unwrap();

    assert_eq!(req.headers().get("Accept").unwrap(), &HeaderValue::from_static("text/html"));
    assert_eq!(req.headers().get("X-Custom-Foo").unwrap(), &HeaderValue::from_static("bar"));
}

#[test]
fn test_header_invalid_key() {
    use http::{Request, header::HeaderValue};

    let result = Request::builder().header("Invalid-Header-Name-@", "value").body(());
    
    assert!(result.is_err());
}

#[test]
fn test_header_invalid_value() {
    use http::{Request, header::HeaderName};

    let req = Request::builder()
        .header("Valid-Header-Name", "valid_value");
    
    let result = req.header("Another-Header", vec![1, 2, 3]); // Trying to use an invalid type.
    
    assert!(result.is_err());
}

