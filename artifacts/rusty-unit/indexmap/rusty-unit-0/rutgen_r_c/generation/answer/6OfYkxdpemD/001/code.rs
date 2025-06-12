// Answer 0

#[test]
fn test_index_map_new() {
    // Test the creation of the IndexMap with no capacity.
    let map: IndexMap<i32, i32> = IndexMap::new();
    // Check that the map is indeed empty.
    assert!(map.core.entries.as_entries().is_empty());
    assert_eq!(map.core.indices.len(), 0);
}

#[test]
fn test_index_map_new_with_capacity() {
    // Ensure that creating a new IndexMap with capacity zero does not fail.
    let map: IndexMap<i32, i32> = IndexMap::with_capacity(0);
    assert!(map.core.entries.as_entries().is_empty());
    assert_eq!(map.core.indices.len(), 0);
}

// Note: Given that the function `new` does not take any parameters,
// there isn't much variability to test. The tests primarily confirm the
// expected behavior is as required.

