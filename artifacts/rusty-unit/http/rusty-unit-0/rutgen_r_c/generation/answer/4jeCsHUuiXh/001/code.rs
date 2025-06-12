// Answer 0

#[test]
fn test_parts_new_default() {
    let parts = Parts::new();
    
    assert_eq!(parts.method, Method::default());
    assert_eq!(parts.uri, Uri::default());
    assert_eq!(parts.version, Version::default());
    assert_eq!(parts.headers, HeaderMap::default());
    assert_eq!(parts.extensions, Extensions::default());
}

#[test]
fn test_parts_default_struct_values() {
    let parts = Parts::new();
    
    // Check the default values directly
    assert!(parts.method == Method::default());
    assert!(parts.uri == Uri::default());
    assert!(parts.version == Version::default());
    assert!(parts.headers == HeaderMap::default());
    assert!(parts.extensions == Extensions::default());
}

