// Answer 0

#[test]
fn test_body_with_valid_args() {
    use crate::{Method, Uri, Version, HeaderMap, HeaderValue, Extensions, Result};

    let builder = Builder::new()
        .method(Method::GET)
        .uri("http://example.com")
        .version(Version::HTTP_1_1);

    let result: Result<Request<()>> = builder.body(());

    assert!(result.is_ok());
}

#[test]
fn test_body_with_invalid_method() {
    use crate::{Method, Result};

    let builder = Builder::new().method("INVALID_METHOD");

    let result: Result<Request<()>> = builder.body(());

    assert!(result.is_err());
}

#[test]
fn test_body_without_uri() {
    let builder = Builder::new().method(Method::GET).version(Version::HTTP_1_1);

    let result: Result<Request<()>> = builder.body(());

    assert!(result.is_ok());
}

#[test]
fn test_body_with_custom_body() {
    let builder = Builder::new()
        .method(Method::POST)
        .uri("http://example.com")
        .version(Version::HTTP_1_1);

    let result: Result<Request<String>> = builder.body("Custom body".to_string());

    assert!(result.is_ok());
}

