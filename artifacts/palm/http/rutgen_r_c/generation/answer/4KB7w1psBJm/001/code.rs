// Answer 0

#[test]
fn test_uri_mut_initialization() {
    struct TestBody;
    let mut request: Request<TestBody> = Request::new(TestBody);
    
    // Create a default Parts instance for initialization
    let parts = Parts {
        method: Method::GET,
        uri: Uri {
            scheme: Scheme::HTTP,
            authority: Authority::from_str("example.com").unwrap(),
            path_and_query: PathAndQuery::from_str("/initial").unwrap(),
        },
        version: Version::HTTP_11,
        headers: HeaderMap::new(),
        extensions: Extensions::new(),
        _priv: (),
    };
    
    // Use from_parts to combine parts and body
    request = Request::from_parts(parts, TestBody);
    
    // Get the mutable reference to the URI
    let uri_mut = request.uri_mut();
    
    // Assert that the URI is as expected
    assert_eq!(uri_mut.path_and_query.as_ref().unwrap().to_string(), "/initial");
}

#[test]
fn test_uri_mut_after_change() {
    struct TestBody;
    let mut request: Request<TestBody> = Request::new(TestBody);
    
    let parts = Parts {
        method: Method::GET,
        uri: Uri {
            scheme: Scheme::HTTP,
            authority: Authority::from_str("example.com").unwrap(),
            path_and_query: PathAndQuery::from_str("/initial").unwrap(),
        },
        version: Version::HTTP_11,
        headers: HeaderMap::new(),
        extensions: Extensions::new(),
        _priv: (),
    };
    
    request = Request::from_parts(parts, TestBody);
    
    // Modify the URI using the mutable reference
    *request.uri_mut() = Uri {
        scheme: Scheme::HTTPS,
        authority: Authority::from_str("example.com").unwrap(),
        path_and_query: PathAndQuery::from_str("/changed").unwrap(),
    };
    
    // Assert that the URI has been changed correctly
    assert_eq!(request.uri().path_and_query.as_ref().unwrap().to_string(), "/changed");
}

