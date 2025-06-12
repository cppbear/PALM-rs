// Answer 0

#[test]
fn test_sort_by_cached_key_empty_map() {
    // Create a new indexmap
    let mut map: indexmap::IndexMap<i32, i32> = indexmap::IndexMap::new();
    
    // Sort with a simple key extraction function
    map.sort_by_cached_key(|_k, v| *v);
    
    // Assert that the map is still empty after sorting
    assert!(map.is_empty());
}

#[test]
fn test_sort_by_cached_key_single_entry() {
    // Create a new indexmap with a single entry
    let mut map: indexmap::IndexMap<i32, i32> = indexmap::IndexMap::new();
    map.insert(1, 10);

    // Sort with a simple key extraction function
    map.sort_by_cached_key(|_k, v| *v);
    
    // Assert that the map contains the same single entry
    assert_eq!(map.len(), 1);
    assert_eq!(map.get(&1), Some(&10));
}

#[test]
fn test_sort_by_cached_key_multiple_entries() {
    // Create a new indexmap with multiple entries
    let mut map: indexmap::IndexMap<i32, i32> = indexmap::IndexMap::new();
    map.insert(2, 20);
    map.insert(1, 15);
    map.insert(3, 10);

    // Sort with a simple key extraction function based on value
    map.sort_by_cached_key(|_k, v| *v);

    // Assert the order after sorting
    let keys: Vec<_> = map.keys().cloned().collect();
    let expected_keys = vec![3, 1, 2]; // Expecting keys in the order of their values
    assert_eq!(keys, expected_keys);
}

#[test]
fn test_sort_by_cached_key_with_same_values() {
    // Create a new indexmap with entries that have the same values
    let mut map: indexmap::IndexMap<i32, i32> = indexmap::IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 10);
    map.insert(3, 10);

    // Sort with a simple key extraction function
    map.sort_by_cached_key(|_k, v| *v);
    
    // Assert that the order remains unchanged
    let keys: Vec<_> = map.keys().cloned().collect();
    let expected_keys = vec![1, 2, 3]; // The order should remain the same
    assert_eq!(keys, expected_keys);
}

