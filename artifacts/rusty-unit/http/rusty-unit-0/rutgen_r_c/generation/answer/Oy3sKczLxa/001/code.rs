// Answer 0

#[test]
fn test_headers_mut_insertion() {
    struct DummyBody;

    let mut request: Request<DummyBody> = Request::new(DummyBody);
    
    // Initialize empty Headers
    let headers_mut = request.headers_mut();
    assert!(headers_mut.entries.is_empty());

    // Insert a header and check that it's non-empty
    let header_name = HeaderName::from_static("host");
    let header_value = HeaderValue::from_static("world");
    headers_mut.insert(header_name.clone(), header_value);

    // Validate that header is inserted
    assert!(!request.headers().entries.is_empty());
    assert_eq!(request.headers().entries.len(), 1);
    
    // Validate the inserted header key and value
    let header = request.headers().entries.first().unwrap();
    assert_eq!(header.name, header_name);
    assert_eq!(&header.value.inner, &HeaderValue::from_static("world").inner);
}

#[test]
#[should_panic]
fn test_headers_mut_panic_on_invalid_mutation() {
    struct DummyBody;

    let mut request: Request<DummyBody> = Request::new(DummyBody);
    
    // Attempt to access headers_mut while it's not mutable could cause a panic
    let headers = request.headers();
    let _header_mutation = headers.insert(HeaderName::from_static("host"), HeaderValue::from_static("world"));
}

