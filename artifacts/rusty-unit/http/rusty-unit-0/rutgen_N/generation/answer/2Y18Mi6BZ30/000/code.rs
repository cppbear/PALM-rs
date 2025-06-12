// Answer 0

#[test]
fn test_post_with_valid_uri() {
    use http::{Request, Builder, Method, Uri};
    use std::convert::TryInto;

    let uri: &str = "https://www.rust-lang.org/";
    let request = Request::post(uri).body(()).unwrap();
    
    assert_eq!(request.method(), Method::POST);
    assert_eq!(request.uri(), uri.parse::<Uri>().unwrap());
}

#[test]
#[should_panic]
fn test_post_with_invalid_uri() {
    use http::{Request, Builder};
   
    let uri: &str = "invalid_uri";
    let _request = Request::post(uri).body(()).unwrap(); // This should panic
}

