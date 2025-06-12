// Answer 0

#[test]
fn test_keys_with_empty_header_map() {
    let map: HeaderMap = HeaderMap::with_capacity(0);
    let keys: Keys<'_, HeaderValue> = map.keys();
    assert_eq!(keys.inner.len(), 0);
}

#[test]
fn test_keys_with_single_entry() {
    let mut map: HeaderMap = HeaderMap::with_capacity(1);
    let key = HeaderName::from_static("key1");
    map.insert(key.clone(), HeaderValue::from_static("value1"));
    let keys: Keys<'_, HeaderValue> = map.keys();
    let keys_vec: Vec<_> = keys.inner.map(|bucket| &bucket.key).collect();
    assert_eq!(keys_vec.len(), 1);
    assert_eq!(keys_vec[0], &key);
}

#[test]
fn test_keys_with_multiple_entries() {
    let mut map: HeaderMap = HeaderMap::with_capacity(3);
    let key1 = HeaderName::from_static("key1");
    let key2 = HeaderName::from_static("key2");
    let key3 = HeaderName::from_static("key3");
    
    map.insert(key1.clone(), HeaderValue::from_static("value1"));
    map.insert(key2.clone(), HeaderValue::from_static("value2"));
    map.insert(key3.clone(), HeaderValue::from_static("value3"));

    let keys: Keys<'_, HeaderValue> = map.keys();
    let keys_vec: Vec<_> = keys.inner.map(|bucket| &bucket.key).collect();
    
    assert_eq!(keys_vec.len(), 3);
    assert!(keys_vec.contains(&&key1));
    assert!(keys_vec.contains(&&key2));
    assert!(keys_vec.contains(&&key3));
}

#[test]
fn test_keys_with_duplicates() {
    let mut map: HeaderMap = HeaderMap::with_capacity(2);
    let key = HeaderName::from_static("duplicate");
    
    map.insert(key.clone(), HeaderValue::from_static("value1"));
    map.append(key.clone(), HeaderValue::from_static("value2")); // Append should create a duplicate key in terms of stored values
    
    let keys: Keys<'_, HeaderValue> = map.keys();
    let keys_vec: Vec<_> = keys.inner.map(|bucket| &bucket.key).collect();

    assert_eq!(keys_vec.len(), 1); // Still should only return one key
    assert_eq!(keys_vec[0], &key);
}

#[test]
fn test_keys_capacity() {
    let capacity = 4;
    let mut map: HeaderMap = HeaderMap::with_capacity(capacity);
    
    // Fill the map to its capacity and beyond to test behavior
    for i in 0..capacity + 2 {
        let key = HeaderName::from_static(&format!("key{}", i));
        map.insert(key.clone(), HeaderValue::from_static("value"));
    }

    let keys: Keys<'_, HeaderValue> = map.keys();
    let keys_vec: Vec<_> = keys.inner.map(|bucket| &bucket.key).collect();
    
    assert_eq!(keys_vec.len(), capacity + 2); // Should reflect actual entries
}

