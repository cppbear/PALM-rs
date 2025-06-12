// Answer 0

#[test]
fn test_swap_remove_index_with_valid_index() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("key1", 1);
    map.insert("key2", 2);
    map.insert("key3", 3);
    
    // Remove the second entry (index 1)
    let result = map.swap_remove_index(1);
    assert_eq!(result, Some(("key2", 2)));
    assert_eq!(map.len(), 2);
    assert!(map.get("key2").is_none());
}

#[test]
fn test_swap_remove_index_with_last_index() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("key1", 1);
    map.insert("key2", 2);
    
    // Remove the last entry (index 1)
    let result = map.swap_remove_index(1);
    assert_eq!(result, Some(("key2", 2)));
    assert_eq!(map.len(), 1);
    assert!(map.get("key2").is_none());
}

#[test]
fn test_swap_remove_index_with_first_index() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("key1", 1);
    map.insert("key2", 2);
    
    // Remove the first entry (index 0)
    let result = map.swap_remove_index(0);
    assert_eq!(result, Some(("key1", 1)));
    assert_eq!(map.len(), 1);
    assert!(map.get("key1").is_none());
}

#[should_panic]
#[test]
fn test_swap_remove_index_with_out_of_bounds_index() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("key1", 1);
    
    // Attempt to remove an entry using an out of bounds index (index 2)
    let _result = map.swap_remove_index(2);
}

