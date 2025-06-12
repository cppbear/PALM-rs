// Answer 0

#[test]
fn test_body_valid_request() {
    use http::{Builder, Method, Uri, Version, Result};

    let builder = Builder::new()
        .method(Method::GET)
        .uri("http://example.com")
        .version(Version::HTTP_11);
        
    let request = builder.body(()).unwrap();

    assert!(request.head.method == Method::GET);
    assert!(request.head.uri.to_string() == "http://example.com");
    assert!(request.head.version == Version::HTTP_11);
}

#[test]
#[should_panic]
fn test_body_invalid_method() {
    use http::{Builder, Result};

    let builder = Builder::new()
        .method("INVALID_METHOD") // Invalid method to trigger panic
        .uri("http://example.com");
        
    let _request = builder.body(()).unwrap();
}

#[test]
#[should_panic]
fn test_body_invalid_uri() {
    use http::{Builder, Result};

    let builder = Builder::new()
        .method(Method::GET)
        .uri("htp://invalid_uri") // Invalid URI to trigger panic
        .version(Version::HTTP_11);
        
    let _request = builder.body(()).unwrap();
}

#[test]
fn test_body_with_extensions() {
    use http::{Builder, Method, Uri, Version, Result, Extensions};

    let builder = Builder::new()
        .method(Method::POST)
        .uri("http://example.com")
        .version(Version::HTTP_11)
        .extension(Extensions::new());

    let request = builder.body(()).unwrap();

    assert!(request.head.method == Method::POST);
    assert!(request.head.uri.to_string() == "http://example.com");
    assert!(request.head.version == Version::HTTP_11);
    assert!(request.head.extensions != Extensions::new());
}

