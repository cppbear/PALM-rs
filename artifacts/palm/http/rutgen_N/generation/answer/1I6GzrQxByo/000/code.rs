// Answer 0

#[test]
fn test_connect_with_valid_uri() {
    use http::{Request, Builder, Method, Uri, Error};
    use std::convert::TryInto;

    let uri: &str = "https://www.rust-lang.org/";
    let request = Request::connect(uri)
        .body(())
        .unwrap();

    assert_eq!(request.method(), Method::CONNECT);
    assert_eq!(request.uri().to_string(), uri);
}

#[test]
#[should_panic]
fn test_connect_with_invalid_uri() {
    use http::{Request, Error};
    
    let uri: &str = "invalid_uri";
    let _request = Request::connect(uri)
        .body(())
        .unwrap();
} 

#[test]
fn test_connect_with_empty_uri() {
    use http::{Request, Builder, Method, Uri, Error};
    use std::convert::TryInto;

    let uri: &str = "";
    let result = Request::connect(uri).body(());

    assert!(result.is_err());
}

#[test]
fn test_connect_with_non_ascii_uri() {
    use http::{Request, Builder, Method, Error};
    
    let uri: &str = "https://example.com/путь";
    let request = Request::connect(uri)
        .body(())
        .unwrap();

    assert_eq!(request.method(), Method::CONNECT);
    assert_eq!(request.uri().to_string(), uri);
}

