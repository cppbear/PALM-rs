// Answer 0

#[test]
fn test_clear_empty_map() {
    let mut map: HeaderMap<HeaderValue> = HeaderMap::with_capacity(10);
    map.clear();
    assert!(map.is_empty());
    assert!(map.capacity() > 0);
}

#[test]
fn test_clear_non_empty_map() {
    let mut map: HeaderMap<HeaderValue> = HeaderMap::with_capacity(10);
    map.insert(HeaderName::from("key1"), HeaderValue::from("value1"));
    map.insert(HeaderName::from("key2"), HeaderValue::from("value2"));
    
    map.clear();
    assert!(map.is_empty());
    assert!(map.capacity() > 0);
}

