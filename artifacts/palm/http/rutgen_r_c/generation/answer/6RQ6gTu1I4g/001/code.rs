// Answer 0

#[test]
fn test_contains_key_empty_map() {
    let map: HeaderMap = HeaderMap::with_capacity(10);
    assert!(!map.contains_key("host"));
}

#[test]
fn test_contains_key_after_insert() {
    let mut map: HeaderMap = HeaderMap::with_capacity(10);
    map.insert("host", "world".parse().unwrap());
    assert!(map.contains_key("host"));
}

#[test]
fn test_contains_key_with_different_case() {
    let mut map: HeaderMap = HeaderMap::with_capacity(10);
    map.insert("host", "world".parse().unwrap());
    assert!(map.contains_key("HOST"));
}

#[test]
fn test_contains_key_non_existent() {
    let mut map: HeaderMap = HeaderMap::with_capacity(10);
    map.insert("host", "world".parse().unwrap());
    assert!(!map.contains_key("user-agent"));
}

#[test]
fn test_contains_key_after_clear() {
    let mut map: HeaderMap = HeaderMap::with_capacity(10);
    map.insert("host", "world".parse().unwrap());
    map.clear();
    assert!(!map.contains_key("host"));
}

#[test]
fn test_contains_key_with_large_capacity() {
    let mut map: HeaderMap = HeaderMap::with_capacity(1024);
    for i in 0..1024 {
        map.insert(format!("key{}", i), "value".parse().unwrap());
    }
    assert!(map.contains_key("key512")); // Check mid-range key
    assert!(!map.contains_key("nonexistent")); // Check non-existent key
}

