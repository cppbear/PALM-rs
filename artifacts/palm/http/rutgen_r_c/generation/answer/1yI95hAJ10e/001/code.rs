// Answer 0

#[test]
fn test_is_empty_when_new() {
    let map: http::HeaderMap = http::HeaderMap::with_capacity(0);
    assert!(map.is_empty());
}

#[test]
fn test_is_empty_after_insert() {
    let mut map = http::HeaderMap::with_capacity(1);
    map.insert(http::HeaderName::from_static("Host"), http::HeaderValue::from_static("example.com"));
    assert!(!map.is_empty());
}

#[test]
fn test_is_empty_after_clear() {
    let mut map = http::HeaderMap::with_capacity(1);
    map.insert(http::HeaderName::from_static("Host"), http::HeaderValue::from_static("example.com"));
    map.clear();
    assert!(map.is_empty());
}

#[test]
fn test_is_empty_after_removal() {
    let mut map = http::HeaderMap::with_capacity(1);
    map.insert(http::HeaderName::from_static("Host"), http::HeaderValue::from_static("example.com"));
    map.remove(http::HeaderName::from_static("Host"));
    assert!(map.is_empty());
}

