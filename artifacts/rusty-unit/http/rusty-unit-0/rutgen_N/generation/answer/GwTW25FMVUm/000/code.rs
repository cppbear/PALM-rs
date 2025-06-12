// Answer 0

#[test]
fn test_request_builder_creation() {
    let request = http::builder()
        .method("GET")
        .uri("https://www.rust-lang.org/")
        .header("X-Custom-Foo", "Bar")
        .body(())
        .unwrap();

    assert_eq!(request.method(), "GET");
    assert_eq!(request.uri(), "https://www.rust-lang.org/");
    assert_eq!(request.headers().get("X-Custom-Foo").unwrap(), "Bar");
}

#[test]
fn test_request_builder_with_invalid_method() {
    let result = http::builder()
        .method("")
        .uri("https://www.rust-lang.org/")
        .header("X-Custom-Foo", "Bar")
        .body(())
        .unwrap_err();

    assert!(result.is_invalid_method());
}

#[test]
fn test_request_builder_with_empty_uri() {
    let result = http::builder()
        .method("GET")
        .uri("")
        .header("X-Custom-Foo", "Bar")
        .body(())
        .unwrap_err();

    assert!(result.is_invalid_uri());
}

#[test]
#[should_panic]
fn test_request_builder_panic_on_missing_body() {
    http::builder()
        .method("GET")
        .uri("https://www.rust-lang.org/")
        .header("X-Custom-Foo", "Bar")
        .body(None)
        .unwrap();
}

