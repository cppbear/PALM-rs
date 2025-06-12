// Answer 0

#[test]
fn test_headers_ref_success() {
    use crate::header::{HeaderMap, HeaderValue, HeaderName};
    use crate::status::StatusCode;
    use crate::version::Version;
    use crate::{Builder, Parts, Extensions, Result};

    let mut builder = Builder::new();
    builder = builder
        .status(StatusCode::OK)
        .version(Version::HTTP_11)
        .header("Accept", HeaderValue::from_str("text/html").unwrap())
        .header("X-Custom-Foo", HeaderValue::from_str("bar").unwrap());

    let result = builder.headers_ref();
    assert!(result.is_some());
    let headers = result.unwrap();
    
    assert_eq!(headers["Accept"], "text/html");
    assert_eq!(headers["X-Custom-Foo"], "bar");
}

#[test]
fn test_headers_ref_failure() {
    use crate::header::{HeaderMap, HeaderValue};
    use crate::status::StatusCode;
    use crate::version::Version;
    use crate::{Builder, Parts, Extensions};

    let builder: Builder = Builder::new(); // No headers added, should yield a Result error

    // Ensuring the headers_ref returns None when there's no successful result
    let result = builder.headers_ref();
    assert!(result.is_none());
}

