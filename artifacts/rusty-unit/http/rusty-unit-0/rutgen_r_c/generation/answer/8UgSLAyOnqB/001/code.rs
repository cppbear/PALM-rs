// Answer 0

#[test]
fn test_header_append_valid() {
    use http::header::{HeaderValue, HeaderName};
    use http::{Builder, Parts, StatusCode, Version, Extensions};
    use std::convert::TryInto;
    
    let builder = Builder::new();

    let result = builder
        .header("Content-Type", "text/html")
        .header("X-Custom-Foo", "bar")
        .header("Content-Length", "0")
        .status(StatusCode::OK)
        .version(Version::HTTP_11);

    let parts = result.inner.unwrap();

    assert_eq!(parts.headers.len(), 3);
    assert!(parts.headers.get("Content-Type").is_some());
    assert!(parts.headers.get("X-Custom-Foo").is_some());
    assert!(parts.headers.get("Content-Length").is_some());
}

#[test]
#[should_panic(expected = "header key is invalid")]
fn test_header_append_invalid_key() {
    use http::header::{HeaderValue};
    use http::{Builder};

    let builder = Builder::new();

    // Attempting to use an invalid header key
    let _ = builder.header("!Invalid-Key", "value").inner.unwrap();
}

#[test]
#[should_panic(expected = "header value is invalid")]
fn test_header_append_invalid_value() {
    use http::header::{HeaderName};
    use http::{Builder};

    let builder = Builder::new();

    // Attempting to use an invalid header value
    let _ = builder.header("Valid-Key", "\x00").inner.unwrap();
}

