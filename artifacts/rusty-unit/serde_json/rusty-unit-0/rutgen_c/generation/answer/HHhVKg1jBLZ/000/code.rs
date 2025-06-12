// Answer 0

#[test]
fn test_map_with_capacity() {
    let capacity = 10;
    let map: Map<String, Value> = Map::with_capacity(capacity);
    
    assert_eq!(map.map.len(), 0); // Ensure the map is initialized empty
    #[cfg(feature = "preserve_order")]
    assert!(map.map.is_empty()); // Test for empty state when capacity is set
}

#[test]
fn test_map_with_capacity_zero() {
    let capacity = 0;
    let map: Map<String, Value> = Map::with_capacity(capacity);
    
    assert_eq!(map.map.len(), 0); // Ensure the map is initialized empty even with capacity 0
    #[cfg(feature = "preserve_order")]
    assert!(map.map.is_empty());
}

