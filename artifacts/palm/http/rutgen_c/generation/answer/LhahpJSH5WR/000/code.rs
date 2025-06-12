// Answer 0

#[test]
fn test_keys_empty_map() {
    struct TestValue;
    
    let map: http::HeaderMap<TestValue> = http::HeaderMap::with_capacity(0);
    let mut keys = map.keys();
    assert_eq!(keys.inner.len(), 0);
}

#[test]
fn test_keys_single_entry() {
    struct TestValue;

    let mut map = http::HeaderMap::with_capacity(10);
    map.insert("Test-Key", TestValue);
    let mut keys = map.keys();
    
    assert_eq!(keys.inner.len(), 1);
}

#[test]
fn test_keys_multiple_entries() {
    struct TestValue;

    let mut map = http::HeaderMap::with_capacity(10);
    map.insert("Key-1", TestValue);
    map.insert("Key-2", TestValue);
    let mut keys = map.keys();
    
    assert_eq!(keys.inner.len(), 2);
}

#[test]
fn test_keys_unique_yield() {
    struct TestValue;

    let mut map = http::HeaderMap::with_capacity(10);
    map.insert("Duplicate-Key", TestValue);
    map.append("Duplicate-Key", TestValue);
    map.insert("Unique-Key", TestValue);
    
    let mut keys = map.keys();
    let mut key_set = std::collections::HashSet::new();
    
    while let Some(key) = keys.inner.next() {
        key_set.insert(key.key);
    }
    
    assert_eq!(key_set.len(), 2); // Should yield unique keys only
}

