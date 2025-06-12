// Answer 0

#[test]
fn test_rebuild_empty_map() {
    struct DummyValue;
    let mut header_map: HeaderMap<DummyValue> = HeaderMap::with_capacity(0);
    header_map.rebuild();
    assert!(header_map.len() == 0);
}

#[test]
fn test_rebuild_single_entry() {
    struct DummyValue;
    let mut header_map: HeaderMap<DummyValue> = HeaderMap::with_capacity(2);
    
    let key = HeaderName { inner: Repr::Custom }; // Assuming an appropriate Repr for HeaderName exists
    header_map.insert(key.clone(), DummyValue);
    header_map.rebuild();
    
    assert!(header_map.len() == 1);
    assert!(header_map.contains_key(&key));
}

#[test]
fn test_rebuild_multiple_entries() {
    struct DummyValue;
    let mut header_map: HeaderMap<DummyValue> = HeaderMap::with_capacity(5);
    
    let key1 = HeaderName { inner: Repr::Custom };
    let key2 = HeaderName { inner: Repr::Custom };
    header_map.insert(key1.clone(), DummyValue);
    header_map.insert(key2.clone(), DummyValue);
    header_map.rebuild();
    
    assert!(header_map.len() == 2);
    assert!(header_map.contains_key(&key1));
    assert!(header_map.contains_key(&key2));
}

#[test]
fn test_rebuild_with_collisions() {
    struct DummyValue;
    let mut header_map: HeaderMap<DummyValue> = HeaderMap::with_capacity(2);
    
    let key1 = HeaderName { inner: Repr::Custom };
    let key2 = HeaderName { inner: Repr::Custom }; // Simulate a hash collision
    header_map.insert(key1.clone(), DummyValue);
    header_map.insert(key2.clone(), DummyValue);
    header_map.rebuild();
    
    assert!(header_map.len() == 2);
    assert!(header_map.contains_key(&key1));
    assert!(header_map.contains_key(&key2));
}

#[test]
#[should_panic]
fn test_rebuild_exceeds_capacity() {
    struct DummyValue;
    let mut header_map: HeaderMap<DummyValue> = HeaderMap::with_capacity(1);
    
    let key = HeaderName { inner: Repr::Custom };
    header_map.insert(key.clone(), DummyValue);
    header_map.try_insert(key.clone(), DummyValue).unwrap(); // Should panic on size exceed
    header_map.rebuild();
}

