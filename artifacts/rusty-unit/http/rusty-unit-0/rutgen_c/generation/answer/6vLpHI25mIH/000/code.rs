// Answer 0

#[test]
fn test_extensions_mut() {
    // Initialize the response with default values
    let mut response: Response<()> = Response::new(());

    // Mutate the extensions
    {
        let extensions = response.extensions_mut();
        // Simulate inserting an extension with a simple string
        let mut any_map = AnyMap::new();
        any_map.insert("hello".to_string());
        extensions.map = Some(Box::new(any_map));
    }

    // Check if the inserted extension is accessible
    let extensions = response.extensions();
    let stored_value = extensions.map.as_ref().and_then(|map| map.get::<String>());
    assert_eq!(stored_value, Some(&"hello".to_string()));
}

#[test]
fn test_extensions_mut_empty() {
    // Initialize the response with default values
    let mut response: Response<()> = Response::new(());

    // Check if the extensions are initialized correctly
    let extensions = response.extensions();
    assert!(extensions.map.is_none());
}

