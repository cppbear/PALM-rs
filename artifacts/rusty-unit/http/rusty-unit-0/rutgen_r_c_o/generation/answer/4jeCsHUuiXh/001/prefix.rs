// Answer 0

#[test]
fn test_new_default_parts() {
    let parts = Parts::new();
}

#[test]
fn test_new_default_parts_with_method_get() {
    let parts = Parts::new();
    assert_eq!(parts.method, Method::default());
}

#[test]
fn test_new_default_parts_with_uri_default() {
    let parts = Parts::new();
    assert_eq!(parts.uri, Uri::default());
}

#[test]
fn test_new_default_parts_with_version_default() {
    let parts = Parts::new();
    assert_eq!(parts.version, Version::default());
}

#[test]
fn test_new_default_parts_with_headers_default() {
    let parts = Parts::new();
    assert_eq!(parts.headers, HeaderMap::default());
}

#[test]
fn test_new_default_parts_with_extensions_default() {
    let parts = Parts::new();
    assert_eq!(parts.extensions, Extensions::default());
}

