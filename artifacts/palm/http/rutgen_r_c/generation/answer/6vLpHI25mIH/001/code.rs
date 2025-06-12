// Answer 0

#[test]
fn test_extensions_mut_initialization() {
    let mut response: Response<()> = Response::new(());
    let extensions_mut = response.extensions_mut();
    assert!(extensions_mut.map.is_none());
}

#[test]
fn test_extensions_mut_insertion_and_retrieval() {
    let mut response: Response<()> = Response::new(());
    response.extensions_mut().map = Some(Box::new(AnyMap::new()));
    response.extensions_mut().map.as_mut().unwrap().insert("hello");
    
    let extensions = response.extensions();
    assert!(extensions.map.is_some());
    assert_eq!(extensions.map.as_ref().unwrap().get::<&str>(), Some(&"hello"));
}

#[test]
fn test_extensions_mut_multiple_insertions() {
    let mut response: Response<()> = Response::new(());
    response.extensions_mut().map = Some(Box::new(AnyMap::new()));
    response.extensions_mut().map.as_mut().unwrap().insert("foo");
    response.extensions_mut().map.as_mut().unwrap().insert("bar");

    let extensions = response.extensions();
    assert!(extensions.map.is_some());
    assert_eq!(extensions.map.as_ref().unwrap().get::<&str>(), Some(&"bar"));
}

#[test]
fn test_extensions_mut_hard_reference() {
    let mut response: Response<()> = Response::new(());
    response.extensions_mut().map = Some(Box::new(AnyMap::new()));
    response.extensions_mut().map.as_mut().unwrap().insert("test");

    {
        let extensions_mut = response.extensions_mut();
        assert_eq!(extensions_mut.map.as_ref().unwrap().get::<&str>(), Some(&"test"));
    }
    
    let extensions = response.extensions();
    assert_eq!(extensions.map.as_ref().unwrap().get::<&str>(), Some(&"test"));
}

#[should_panic]
fn test_extensions_mut_panic_on_invalid_access() {
    let mut response: Response<()> = Response::new(());
    let extensions_mut = response.extensions_mut();
    extensions_mut.map = None; // Simulating a panic case by invalidating map
    let _ = extensions_mut.map.as_mut().unwrap(); // This will panic
}

