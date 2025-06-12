// Answer 0

#[test]
fn test_get_empty_map() {
    let mut map: HeaderMap<HeaderValue> = HeaderMap::with_capacity(10);
    assert!(map.get("host").is_none());
}

#[test]
fn test_get_single_value() {
    let mut map: HeaderMap<HeaderValue> = HeaderMap::with_capacity(10);
    let host_key = HeaderName::from_static("host");
    map.insert(host_key.clone(), "hello".parse().unwrap());
    assert_eq!(map.get(host_key).unwrap(), &"hello");
}

#[test]
fn test_get_case_insensitivity() {
    let mut map: HeaderMap<HeaderValue> = HeaderMap::with_capacity(10);
    let host_key = HeaderName::from_static("host");
    map.insert(host_key.clone(), "hello".parse().unwrap());
    assert_eq!(map.get("host").unwrap(), &"hello");
}

#[test]
fn test_get_multiple_values() {
    let mut map: HeaderMap<HeaderValue> = HeaderMap::with_capacity(10);
    let host_key = HeaderName::from_static("host");
    map.insert(host_key.clone(), "hello".parse().unwrap());
    map.append(host_key.clone(), "world".parse().unwrap());
    assert_eq!(map.get("host").unwrap(), &"hello");
}

#[test]
fn test_get_non_existent_key() {
    let mut map: HeaderMap<HeaderValue> = HeaderMap::with_capacity(10);
    let host_key = HeaderName::from_static("host");
    assert!(map.get(host_key).is_none());
}

