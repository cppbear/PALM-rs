// Answer 0

#[test]
fn test_headers_mut_insertion() {
    use http::{Response, header::{HeaderMap, HeaderValue, HOST}};
    
    let mut response: Response<()> = Response::default();
    response.headers_mut().insert(HOST, HeaderValue::from_static("world"));
    
    assert!(!response.headers().is_empty());
    assert_eq!(response.headers().get(HOST).unwrap(), &HeaderValue::from_static("world"));
}

#[test]
fn test_headers_mut_empty_initial() {
    use http::{Response, header::HeaderValue};
    
    let response: Response<()> = Response::default();
    
    assert!(response.headers().is_empty());
}

#[test]
fn test_headers_mut_removal() {
    use http::{Response, header::{HeaderMap, HeaderValue, HOST}};
    
    let mut response: Response<()> = Response::default();
    response.headers_mut().insert(HOST, HeaderValue::from_static("world"));
    
    response.headers_mut().remove(HOST);
    
    assert!(response.headers().is_empty());
}

