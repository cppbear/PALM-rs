// Answer 0

#[test]
fn test_extensions_mut_with_valid_builder() {
    // Create a valid Builder instance
    let mut builder = Builder::new();
    
    // Assuming we can use the methods to setup a valid Parts instance
    builder = builder.status(StatusCode::OK);
    builder = builder.version(Version::HTTP_11);
    
    // Initialize extensions
    {
        let mut extensions = builder.extensions_mut().unwrap();
        assert!(extensions.map.is_none()); // Check that extensions are initially None
        
        // Insert and check custom types in extensions
        extensions.insert("My Extension");
        assert_eq!(extensions.map.as_ref().unwrap().get::<&str>(), Some(&"My Extension"));
        
        extensions.insert(5u32);
        assert_eq!(extensions.map.as_ref().unwrap().get::<u32>(), Some(&5u32));
    }
}

#[test]
fn test_extensions_mut_with_error_in_builder() {
    // Create a Builder instance that leads to an error
    let builder = Builder { inner: Err(Error { inner: ErrorKind::SomeError }) };
    
    // Attempt to get mutable extensions - this should return None
    let extensions = builder.extensions_mut();
    assert!(extensions.is_none());
}

