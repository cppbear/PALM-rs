// Answer 0

#[test]
fn test_new_parts_default() {
    let parts = Parts::new();
    assert_eq!(parts.status, StatusCode::default());
    assert_eq!(parts.version, Version::default());
    assert_eq!(parts.headers, HeaderMap::default());
    assert_eq!(parts.extensions, Extensions::default());
}

#[test]
fn test_new_parts_non_empty() {
    let parts = Parts {
        status: StatusCode::default(),
        version: Version::default(),
        headers: HeaderMap::try_with_capacity(2).expect("should succeed"),
        extensions: Extensions::default(),
        _priv: (),
    };
    assert_eq!(parts.headers.entries.len(), 0);
}

#[test]
fn test_new_parts_with_custom_values() {
    let custom_status = StatusCode::OK; // Assuming OK is available
    let custom_version = Version::default();
    let custom_headers = HeaderMap::default();
    let custom_extensions = Extensions::default();
    
    let parts = Parts {
        status: custom_status,
        version: custom_version,
        headers: custom_headers,
        extensions: custom_extensions,
        _priv: (),
    };
    
    assert_eq!(parts.status, custom_status);
    assert_eq!(parts.version, custom_version);
    assert_eq!(parts.headers, custom_headers);
    assert_eq!(parts.extensions, custom_extensions);
}

