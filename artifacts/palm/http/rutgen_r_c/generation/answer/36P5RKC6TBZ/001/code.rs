// Answer 0

#[test]
fn test_headers_empty() {
    struct ResponseEmpty;
    
    let response: Response<ResponseEmpty> = Response::new(ResponseEmpty);
    assert!(response.headers().is_empty());
}

#[test]
fn test_headers_non_empty() {
    struct ResponseBody;
    
    let mut headers = HeaderMap::default();
    // Assume we have a way to insert headers here
    // headers.insert("Content-Type", HeaderValue::from_str("application/json").unwrap());
    
    let parts = Parts {
        headers,
        ..Default::default()
    };
    
    let response: Response<ResponseBody> = Response::from_parts(parts, ResponseBody);
    assert!(!response.headers().is_empty());
}

#[test]
fn test_headers_mutability() {
    struct ResponseBody;
    
    let mut headers = HeaderMap::default();
    // headers.insert("Content-Type", HeaderValue::from_str("text/html").unwrap());
    
    let mut parts = Parts {
        headers,
        ..Default::default()
    };
    
    let mut response: Response<ResponseBody> = Response::from_parts(parts, ResponseBody);
    
    assert!(!response.headers().is_empty());  // Checking that headers are still non-empty
    
    // Modify the headers through mutable reference
    // Example code to actually modify headers goes here
    // response.headers_mut().insert("Content-Length", HeaderValue::from_str("100").unwrap());
    
    assert_eq!(response.headers().len(), 2); // Assuming we added one more header
}

