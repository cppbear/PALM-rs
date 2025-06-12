// Answer 0

#[test]
fn test_clear_with_entries() {
    struct TestValue;
    let mut map = HeaderMap::<TestValue>::with_capacity(10);
    
    // Simulate the insertion of some entries
    map.indices = vec![Pos::new(0, HashValue(1)).into(), Pos::new(1, HashValue(2)).into()].into_boxed_slice();
    map.entries.push(Bucket { hash: HashValue(1), key: HeaderName::from("key1"), value: TestValue {}, links: None });
    map.entries.push(Bucket { hash: HashValue(2), key: HeaderName::from("key2"), value: TestValue {}, links: None });
    
    assert!(!map.is_empty());
    
    map.clear();
    
    assert!(map.is_empty());
    assert!(map.capacity() > 0);
    assert_eq!(map.entries.len(), 0);
    assert_eq!(map.extra_values.len(), 0);
    assert_eq!(map.danger, Danger::Green);
}

#[test]
fn test_clear_empty_map() {
    struct TestValue;
    let mut map = HeaderMap::<TestValue>::with_capacity(10);
    
    // Empty map should have no entries
    assert!(map.is_empty());
    
    map.clear();
    
    // Should remain empty after clear
    assert!(map.is_empty());
    assert!(map.capacity() > 0); // Check that capacity is still greater than 0
}

#[test]
fn test_clear_with_none_indices() {
    struct TestValue;
    let mut map = HeaderMap::<TestValue>::with_capacity(10);
    
    // Initialize indices as none
    map.indices = vec![Pos::none(); 10].into_boxed_slice(); // All indices should be 'none'
    
    map.clear();
    
    // Ensure map is still cleared correctly
    assert!(map.is_empty());
    assert!(map.capacity() > 0);
    assert_eq!(map.entries.len(), 0);
    assert_eq!(map.extra_values.len(), 0);
    assert_eq!(map.danger, Danger::Green);
}

