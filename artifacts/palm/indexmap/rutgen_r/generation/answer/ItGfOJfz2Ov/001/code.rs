// Answer 0

#[test]
fn test_with_capacity_and_hasher_non_zero() {
    use indexmap::IndexMap;
    use std::collections::hash_map::RandomState;

    // Instantiate a hash builder
    let hash_builder = RandomState::new();
    
    // Test with a small non-zero capacity
    let map_small = IndexMap::with_capacity_and_hasher(1, hash_builder.clone());
    assert_eq!(map_small.len(), 0); // Ensure it's created with the correct initial state
    
    // Test with a moderate capacity
    let map_moderate = IndexMap::with_capacity_and_hasher(100, hash_builder.clone());
    assert_eq!(map_moderate.len(), 0);
    
    // Test with a large capacity
    let map_large = IndexMap::with_capacity_and_hasher(1_000_000, hash_builder.clone());
    assert_eq!(map_large.len(), 0);
}

#[test]
#[should_panic]
fn test_with_capacity_and_hasher_zero() {
    use indexmap::IndexMap;
    use std::collections::hash_map::RandomState;

    // Attempt to call the function with zero capacity
    let hash_builder = RandomState::new();
    let _ = IndexMap::with_capacity_and_hasher(0, hash_builder);
}

