// Answer 0

#[test]
fn test_headers_mut_initialization() {
    let mut response: Response<()> = Response::default();
    let headers_mut = response.headers_mut();
    assert!(headers_mut.is_empty());
}

#[test]
fn test_headers_mut_insert() {
    let mut response: Response<()> = Response::default();
    {
        let headers_mut = response.headers_mut();
        headers_mut.insert(HOST, HeaderValue::from_static("world"));
    }
    
    assert!(!response.headers().is_empty());
}

#[test]
fn test_headers_mut_multiple_inserts() {
    let mut response: Response<()> = Response::default();
    {
        let headers_mut = response.headers_mut();
        headers_mut.insert(HOST, HeaderValue::from_static("world"));
        headers_mut.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
    }
    
    assert_eq!(response.headers().len(), 2);
}

#[test]
fn test_headers_mut_removal() {
    let mut response: Response<()> = Response::default();
    {
        let headers_mut = response.headers_mut();
        headers_mut.insert(HOST, HeaderValue::from_static("world"));
        headers_mut.remove(HOST);
    }
    
    assert!(response.headers().is_empty());
}

#[test]
#[should_panic]
fn test_headers_mut_panic_on_invalid_action() {
    let mut response: Response<()> = Response::default();
    {
        let headers_mut = response.headers_mut();
        headers_mut.insert(HOST, HeaderValue::from_static("world"));
        // Attempt to mutate and invalidate reference should trigger a panic
        std::mem::drop(headers_mut);
        headers_mut.insert(CONTENT_TYPE, HeaderValue::from_static("application/json")); // invalid usage
    }
}

