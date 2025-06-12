// Answer 0

#[test]
fn test_get_with_valid_uri() {
    use http::{Request, Builder, Method, Uri};
    use std::convert::TryInto;

    let uri = "https://www.rust-lang.org/";
    let request = Request::get(uri)
        .body(())
        .unwrap();
    
    assert_eq!(request.method(), Method::GET);
    assert_eq!(request.uri().to_string(), uri);
}

#[test]
#[should_panic]
fn test_get_with_invalid_uri() {
    use http::{Request, Builder, Method};
    
    // Supplying an invalid URI triggering panic on unwrap
    let invalid_uri = "invalid_uri";
    let _request = Request::get(invalid_uri)
        .body(())
        .unwrap();
}

#[test]
fn test_get_with_empty_uri() {
    use http::{Request, Builder, Method, Uri};
    use std::convert::TryInto;

    let uri = "";
    let request = Request::get(uri)
        .body(())
        .unwrap();
    
    assert_eq!(request.method(), Method::GET);
    assert_eq!(request.uri().to_string(), uri);
}

