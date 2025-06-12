// Answer 0

#[test]
fn test_delete_with_valid_uri() {
    use http::{Request, Builder, Method, Uri};
    use std::convert::TryInto;

    let uri = "https://www.rust-lang.org/";
    let request = Request::delete(uri)
        .body(())
        .unwrap();

    assert_eq!(request.method(), Method::DELETE);
    assert_eq!(request.uri().to_string(), uri);
}

#[test]
#[should_panic]
fn test_delete_with_invalid_uri() {
    use http::{Request, Builder};
    use std::convert::TryInto;

    let invalid_uri = "invalid_uri";
    let _request = Request::delete(invalid_uri)
        .body(())
        .unwrap(); // This should panic due to invalid URI
}

#[test]
fn test_delete_with_empty_uri() {
    use http::{Request, Builder};
    use std::convert::TryInto;

    let uri = "";
    let request = Request::delete(uri)
        .body(())
        .unwrap();

    assert_eq!(request.method(), Method::DELETE);
    assert_eq!(request.uri().to_string(), uri);
}

