// Answer 0

#[test]
fn test_extensions_initially_empty() {
    struct EmptyBody;
    
    let request: Request<EmptyBody> = Request::new(EmptyBody);
    assert!(request.extensions().map.is_none());
}

#[test]
fn test_extensions_after_adding() {
    struct TestBody;
    
    let mut request: Request<TestBody> = Request::new(TestBody);

    // Simulate adding extensions (not implemented, just conceptual)
    let extensions = Extensions {
        map: Some(Box::new(AnyMap::new())),
    };
    
    request.head.extensions = extensions;
    assert!(request.extensions().map.is_some());
} 

#[test]
fn test_extensions_mutability() {
    struct MutableBody;

    let mut request: Request<MutableBody> = Request::new(MutableBody);
    let extensions = Extensions::default();
    
    *request.extensions_mut() = extensions;
    assert!(request.extensions().map.is_none());
}

