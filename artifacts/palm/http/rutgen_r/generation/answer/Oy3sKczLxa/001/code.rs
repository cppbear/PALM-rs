// Answer 0

#[test]
fn test_headers_mut_insertion() {
    use http::*;
    use http::header::*;

    let mut request: Request<()> = Request::default();
    {
        let headers = request.headers_mut();
        headers.insert(HOST, HeaderValue::from_static("world"));
        assert!(!request.headers().is_empty());
    }
}

#[test]
fn test_headers_mut_empty() {
    use http::*;
    use http::header::*;

    let mut request: Request<()> = Request::default();
    let headers = request.headers_mut();
    assert!(headers.is_empty());
}

#[test]
fn test_headers_mut_multiple_insertions() {
    use http::*;
    use http::header::*;

    let mut request: Request<()> = Request::default();
    {
        let headers = request.headers_mut();
        headers.insert(HOST, HeaderValue::from_static("world"));
        headers.insert(USER_AGENT, HeaderValue::from_static("test-agent"));
        assert_eq!(request.headers().len(), 2);
    }
}

#[test]
fn test_headers_mut_removal() {
    use http::*;
    use http::header::*;

    let mut request: Request<()> = Request::default();
    {
        let headers = request.headers_mut();
        headers.insert(HOST, HeaderValue::from_static("world"));
        assert_eq!(request.headers().len(), 1);
        headers.remove(HOST);
        assert!(request.headers().is_empty());
    }
}

