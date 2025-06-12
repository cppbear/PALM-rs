// Answer 0

#[test]
fn test_header_with_valid_key_value() {
    use http::header::{HeaderValue, HeaderName};
    
    let builder = Builder::new();
    let result = builder.header("Accept", "text/html");

    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_header_with_invalid_key() {
    use http::header::{HeaderValue, HeaderName};

    let builder = Builder::new();
    let result = builder.header("", "text/html"); // Empty header name should panic

    assert!(result.is_err());
}

#[test]
fn test_header_with_valid_multiple_headers() {
    use http::header::{HeaderValue, HeaderName};

    let builder = Builder::new();
    let builder = builder.header("Accept", "text/html").header("X-Custom-Foo", "bar");

    assert!(builder.is_ok());
}

#[test]
fn test_header_with_invalid_value() {
    use http::header::{HeaderValue, HeaderName};

    let builder = Builder::new();
    let result = builder.header("Accept", "");

    assert!(result.is_err());
}

