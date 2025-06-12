// Answer 0

#[test]
fn test_headers_mut_inserts_header() {
    use http::*;
    use http::header::*;

    let mut request: Request<()> = Request::default();
    request.headers_mut().insert(HOST, HeaderValue::from_static("world"));
    assert!(!request.headers().is_empty());
}

#[test]
fn test_headers_mut_empty_initially() {
    use http::*;
    use http::header::*;

    let mut request: Request<()> = Request::default();
    assert!(request.headers().is_empty());
}

#[test]
fn test_headers_mut_multiple_inserts() {
    use http::*;
    use http::header::*;

    let mut request: Request<()> = Request::default();
    request.headers_mut().insert(HOST, HeaderValue::from_static("world"));
    request.headers_mut().insert(CONNECTION, HeaderValue::from_static("keep-alive"));
    
    assert_eq!(request.headers().len(), 2);
}

#[test]
fn test_headers_mut_removes_header() {
    use http::*;
    use http::header::*;

    let mut request: Request<()> = Request::default();
    request.headers_mut().insert(HOST, HeaderValue::from_static("world"));
    
    request.headers_mut().remove(HOST);
    
    assert!(request.headers().is_empty());
}

