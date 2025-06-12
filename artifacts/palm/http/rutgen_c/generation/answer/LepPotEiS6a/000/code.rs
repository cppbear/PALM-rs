// Answer 0

#[test]
fn test_keys_len_empty() {
    struct TestValue;

    let map: HeaderMap<TestValue> = HeaderMap::with_capacity(10);
    assert_eq!(map.keys_len(), 0);
}

#[test]
fn test_keys_len_single_insert() {
    struct TestValue;

    let mut map: HeaderMap<TestValue> = HeaderMap::with_capacity(10);
    
    map.insert(HeaderName::from_static("key1"), TestValue);
    assert_eq!(map.keys_len(), 1);
}

#[test]
fn test_keys_len_multiple_inserts_same_key() {
    struct TestValue;

    let mut map: HeaderMap<TestValue> = HeaderMap::with_capacity(10);
    
    map.insert(HeaderName::from_static("key1"), TestValue);
    map.insert(HeaderName::from_static("key1"), TestValue); // Inserting again should not increase keys_len
    assert_eq!(map.keys_len(), 1);
}

#[test]
fn test_keys_len_multiple_different_keys() {
    struct TestValue;

    let mut map: HeaderMap<TestValue> = HeaderMap::with_capacity(10);
    
    map.insert(HeaderName::from_static("key1"), TestValue);
    map.insert(HeaderName::from_static("key2"), TestValue);
    assert_eq!(map.keys_len(), 2);
}

#[test]
fn test_keys_len_multiple_inserts_with_separate_values() {
    struct TestValue;

    let mut map: HeaderMap<TestValue> = HeaderMap::with_capacity(10);
    
    map.insert(HeaderName::from_static("key1"), TestValue);
    map.insert(HeaderName::from_static("key2"), TestValue);
    map.insert(HeaderName::from_static("key1"), TestValue); // Still should count the distinct key
    assert_eq!(map.keys_len(), 2);
}

