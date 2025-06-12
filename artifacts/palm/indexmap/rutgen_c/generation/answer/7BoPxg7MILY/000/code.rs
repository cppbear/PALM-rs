// Answer 0

#[cfg(test)]
fn test_insert_full() {
    struct SimpleHasher;
    
    impl BuildHasher for SimpleHasher {
        type Hasher = std::collections::hash_map::RandomState;
        
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut map: IndexMap<i32, String, SimpleHasher> = IndexMap::new();
    
    // Test insertion of a new key-value pair
    let (index, old_value) = map.insert_full(1, "value1".to_string());
    assert_eq!(index, 0);
    assert_eq!(old_value, None);

    // Test insertion of an equivalent key-value pair
    let (index, old_value) = map.insert_full(1, "value2".to_string());
    assert_eq!(index, 0);
    assert_eq!(old_value, Some("value1".to_string()));

    // Test insertion of another new key-value pair
    let (index, old_value) = map.insert_full(2, "value3".to_string());
    assert_eq!(index, 1);
    assert_eq!(old_value, None);

    // Validate the order and values in the map
    assert_eq!(map.entry(1).get(), Some(&"value2".to_string()));
    assert_eq!(map.entry(2).get(), Some(&"value3".to_string()));
}

