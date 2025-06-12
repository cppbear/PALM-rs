// Answer 0

#[test]
fn test_builder_creation() {
    let builder = http::builder();
    assert!(builder.is_ok());
}

#[test]
fn test_builder_with_valid_method() {
    use http::{Request, Method};

    let request = http::builder()
        .method(Method::GET)
        .uri("https://www.rust-lang.org/")
        .body(())
        .unwrap();

    assert_eq!(request.method(), &Method::GET);
}

#[test]
fn test_builder_with_uri() {
    use http::{Request};

    let request = http::builder()
        .method("POST")
        .uri("https://www.rust-lang.org/")
        .body(())
        .unwrap();
    
    assert_eq!(request.uri().to_string(), "https://www.rust-lang.org/");
}

#[test]
#[should_panic]
fn test_builder_with_invalid_method() {
    use http::Method;

    let _request = http::builder()
        .method(Method::from_bytes(&b"INVALID"[..]).unwrap())
        .uri("https://www.rust-lang.org/")
        .body(())
        .unwrap();
}

#[test]
fn test_builder_with_header() {
    use http::{Request, HeaderMap};

    let request = http::builder()
        .method("GET")
        .uri("https://www.rust-lang.org/")
        .header("X-Custom-Foo", "Bar")
        .body(())
        .unwrap();
    
    let headers: &HeaderMap = request.headers();
    assert_eq!(headers.get("X-Custom-Foo").unwrap(), "Bar");
}

