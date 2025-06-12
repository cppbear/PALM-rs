// Answer 0

#[test]
fn test_clear_empty_map() {
    let mut map: indexmap::IndexMap<i32, i32> = indexmap::IndexMap::new();
    map.clear();
    assert_eq!(map.len(), 0);
}

#[test]
fn test_clear_non_empty_map() {
    let mut map: indexmap::IndexMap<i32, i32> = indexmap::IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.clear();
    assert_eq!(map.len(), 0);
}

#[test]
fn test_clear_map_with_capacity() {
    let mut map: indexmap::IndexMap<i32, i32> = indexmap::IndexMap::with_capacity(10);
    map.insert(1, 10);
    map.insert(2, 20);
    assert!(map.capacity() >= 10);
    map.clear();
    assert_eq!(map.len(), 0);
    assert!(map.capacity() >= 10); // Capacity should be preserved
}

#[test]
fn test_clear_map_with_duplicate_keys() {
    let mut map: indexmap::IndexMap<i32, i32> = indexmap::IndexMap::new();
    map.insert(1, 10);
    map.insert(1, 20); // Duplicate key, latest value will overwrite
    map.clear();
    assert_eq!(map.len(), 0);
}

