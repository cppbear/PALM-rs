// Answer 0

#[test]
fn test_parts_new() {
    let parts = Parts::new();
    
    assert_eq!(parts.method, Method::default());
    assert_eq!(parts.uri, Uri::default());
    assert_eq!(parts.version, Version::default());
    assert_eq!(parts.headers, HeaderMap::default());
    assert_eq!(parts.extensions, Extensions::default());
}

#[test]
fn test_parts_new_non_default_values() {
    #[derive(Clone, Default, PartialEq)]
    struct TestMethod(Method);
    
    #[derive(Clone, Default, PartialEq)]
    struct TestVersion(Version);
    
    #[derive(Clone, Default, PartialEq)]
    struct TestUri(Uri);
    
    let method = TestMethod(Method::default());
    let uri = TestUri(Uri::default());
    let version = TestVersion(Version::default());
    let headers = HeaderMap::default();
    let extensions = Extensions::default();
    
    let parts = Parts {
        method: method.0,
        uri: uri.0,
        version: version.0,
        headers,
        extensions,
        _priv: (),
    };
    
    assert_eq!(parts.method, Method::default());
    assert_eq!(parts.uri, Uri::default());
    assert_eq!(parts.version, Version::default());
    assert_eq!(parts.headers, HeaderMap::default());
    assert_eq!(parts.extensions, Extensions::default());
}

