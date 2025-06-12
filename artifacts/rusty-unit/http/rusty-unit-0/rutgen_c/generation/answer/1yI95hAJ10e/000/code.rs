// Answer 0

#[test]
fn test_is_empty_on_new_header_map() {
    let map: HeaderMap = HeaderMap::with_capacity(0);
    assert!(map.is_empty());
}

#[test]
fn test_is_empty_after_insert() {
    let mut map: HeaderMap<String> = HeaderMap::with_capacity(1);
    map.insert("key", "value".to_string());
    assert!(!map.is_empty());
}

#[test]
fn test_is_empty_after_clear() {
    let mut map: HeaderMap<String> = HeaderMap::with_capacity(1);
    map.insert("key", "value".to_string());
    map.clear();
    assert!(map.is_empty());
}

