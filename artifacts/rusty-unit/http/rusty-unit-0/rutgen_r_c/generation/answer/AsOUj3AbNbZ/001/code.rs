// Answer 0

#[test]
fn test_extensions_mut() {
    // Create an instance of Request with a default body type
    let mut request: Request<()> = Request::new(());

    // Access mutable reference to extensions
    let extensions = request.extensions_mut();

    // Test by inserting a value into the extensions
    extensions.map = Some(Box::new(AnyMap::default()));
    extensions.map.as_mut().unwrap().insert("hello");
    
    // Assert the value has been properly inserted
    assert_eq!(request.extensions().map.as_ref().unwrap().get::<&str>(), Some(&"hello"));
}

#[test]
fn test_extensions_mut_default() {
    // Create another request instance with default body
    let mut request: Request<()> = Request::new(());

    // Access mutable reference to extensions
    let extensions = request.extensions_mut();

    // Ensure the default extensions are initialized correctly
    assert!(extensions.map.is_none());
    
    // Now, insert a new value
    extensions.map = Some(Box::new(AnyMap::default()));
    extensions.map.as_mut().unwrap().insert("world");
    
    // Assert the value has been properly inserted
    assert_eq!(request.extensions().map.as_ref().unwrap().get::<&str>(), Some(&"world"));
}

#[test]
#[should_panic]
fn test_extensions_mut_panic() {
    // Create a request instance
    let mut request: Request<()> = Request::new(());

    // Access mutable reference to extensions
    let extensions = request.extensions_mut();
    
    // Try to access map without initialization
    assert!(extensions.map.is_none());
    let _ = extensions.map.as_ref().unwrap().get::<&str>(); // This should panic
}

