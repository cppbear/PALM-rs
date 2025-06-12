// Answer 0

#[test]
fn test_header_with_valid_key_value() {
    use http::*;
    use http::header::{HeaderName, HeaderValue};

    let response = Response::builder()
        .header("Content-Type", "text/html")
        .header("X-Custom-Foo", "bar")
        .header("content-length", 0)
        .body(())
        .unwrap();

    assert_eq!(response.headers().get("Content-Type").unwrap(), HeaderValue::from_static("text/html"));
    assert_eq!(response.headers().get("X-Custom-Foo").unwrap(), HeaderValue::from_static("bar"));
    assert_eq!(response.headers().get("content-length").unwrap(), HeaderValue::from_static("0"));
}

#[test]
#[should_panic]
fn test_header_with_invalid_key() {
    use http::*;
    use http::header::{HeaderName, HeaderValue};

    let _response = Response::builder()
        .header(HeaderName::from_static("Invalid-Key"), "valid-value")
        .header("Another-Key", "another-value")
        .body(())
        .unwrap();
}

#[test]
fn test_header_with_multiple_headers() {
    use http::*;
    use http::header::{HeaderName, HeaderValue};

    let response = Response::builder()
        .header("X-Foo", "foo")
        .header("X-Bar", "bar")
        .header("X-Baz", "baz")
        .body(())
        .unwrap();

    assert_eq!(response.headers().get("X-Foo").unwrap(), HeaderValue::from_static("foo"));
    assert_eq!(response.headers().get("X-Bar").unwrap(), HeaderValue::from_static("bar"));
    assert_eq!(response.headers().get("X-Baz").unwrap(), HeaderValue::from_static("baz"));
}

