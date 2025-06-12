// Answer 0

#[test]
fn test_headers_mut_inserts_header() {
    #[derive(Default)]
    struct DummyBody;

    let mut response: Response<DummyBody> = Response::new(DummyBody::default());
    response.headers_mut().insert(HeaderName::from_static("host"), HeaderValue { inner: Bytes::from_static(b"world"), is_sensitive: false });
    
    assert!(!response.headers().entries.is_empty());
}

#[test]
fn test_headers_mut_empty_before_insert() {
    #[derive(Default)]
    struct DummyBody;

    let response: Response<DummyBody> = Response::new(DummyBody::default());
    
    assert!(response.headers().entries.is_empty());
}

#[test]
fn test_headers_mut_multiple_inserts() {
    #[derive(Default)]
    struct DummyBody;

    let mut response: Response<DummyBody> = Response::new(DummyBody::default());
    response.headers_mut().insert(HeaderName::from_static("host"), HeaderValue { inner: Bytes::from_static(b"world"), is_sensitive: false });
    response.headers_mut().insert(HeaderName::from_static("user-agent"), HeaderValue { inner: Bytes::from_static(b"test-agent"), is_sensitive: false });
    
    assert_eq!(response.headers().entries.len(), 2);
}

#[test]
fn test_headers_mut_remains_mutable() {
    #[derive(Default)]
    struct DummyBody;

    let mut response: Response<DummyBody> = Response::new(DummyBody::default());
    {
        let headers = response.headers_mut();
        headers.insert(HeaderName::from_static("header1"), HeaderValue { inner: Bytes::from_static(b"value1"), is_sensitive: false });
        assert_eq!(headers.entries.len(), 1);
    }

    assert_eq!(response.headers().entries.len(), 1);
}

