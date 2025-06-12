// Answer 0

#[test]
fn test_extensions_mut_inserts_value() {
    struct DummyBody;
    
    let mut request: Request<DummyBody> = Request::new(DummyBody);
    request.extensions_mut().insert("hello");

    assert_eq!(request.extensions().get::<&str>(), Some(&"hello"));
}

#[test]
fn test_extensions_mut_empty() {
    struct DummyBody;

    let request: Request<DummyBody> = Request::new(DummyBody);
    
    assert_eq!(request.extensions().get::<&str>(), None);
}

#[test]
#[should_panic]
fn test_extensions_mut_panic_on_non_existent_value() {
    struct DummyBody;

    let request: Request<DummyBody> = Request::new(DummyBody);
    request.extensions().get::<&str>().unwrap(); // This should panic since there is no value
}

