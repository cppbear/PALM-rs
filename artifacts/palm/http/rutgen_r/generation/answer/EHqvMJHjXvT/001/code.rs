// Answer 0

#[test]
fn test_parts_new() {
    use http::{Parts, StatusCode, Version, HeaderMap, Extensions};

    let parts = Parts::new();

    assert_eq!(parts.status, StatusCode::default());
    assert_eq!(parts.version, Version::default());
    assert_eq!(parts.headers, HeaderMap::default());
    assert_eq!(parts.extensions, Extensions::default());
}

