// Answer 0

#[test]
fn test_uri_mut() {
    struct TestBody;

    let mut request: Request<TestBody> = Request::new(TestBody);
    
    let test_uri = Uri {
        scheme: Some(Scheme::Http),
        authority: Some(Authority::from_static("example.com")),
        path_and_query: Some(PathAndQuery::from_static("/hello")),
        _priv: (),
    };

    *request.uri_mut() = test_uri.clone();
    assert_eq!(*request.uri(), test_uri);
}

#[test]
fn test_make_uri_mut_empty() {
    struct TestBody;

    let mut request: Request<TestBody> = Request::new(TestBody);
    
    let empty_uri = Uri::default();

    *request.uri_mut() = empty_uri.clone();
    assert_eq!(*request.uri(), empty_uri);
}

