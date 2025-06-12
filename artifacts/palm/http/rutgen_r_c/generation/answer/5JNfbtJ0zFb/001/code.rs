// Answer 0

#[test]
fn test_extensions_ref_with_valid_extensions() {
    struct TestMethod;
    struct TestUri;
    
    let req = Builder::new()
        .method(TestMethod)
        .uri(TestUri)
        .extension("My Extension")
        .extension(5u32);

    let extensions = req.extensions_ref().unwrap();
    assert_eq!(extensions.get::<&'static str>(), Some(&"My Extension"));
    assert_eq!(extensions.get::<u32>(), Some(&5u32));
}

#[test]
fn test_extensions_ref_with_no_extensions() {
    struct TestMethod;
    struct TestUri;

    let req = Builder::new()
        .method(TestMethod)
        .uri(TestUri);

    let extensions = req.extensions_ref();
    assert!(extensions.is_none());
}

#[test]
fn test_extensions_ref_with_error() {
    struct TestMethod;
    struct TestUri;
    
    let req = Builder::new()
        .method(TestMethod)
        .uri(TestUri)
        .extension("My Extension")
        .and_then(|_| Err(Error { inner: ErrorKind::from_str("error") }));

    let extensions = req.extensions_ref();
    assert!(extensions.is_none());
}

