// Answer 0

#[test]
fn test_map_new() {
    // Test to ensure that a new Map is created successfully.
    let map: Map<String, Value> = Map::new();
    
    // Assert that the map is empty upon creation
    assert_eq!(map.len(), 0);
    assert!(map.is_empty());

    // Further assert that the internal structure is initialized as expected
    #[cfg(not(feature = "preserve_order"))]
    {
        let _: BTreeMap<String, Value> = map.map;
    }
    #[cfg(feature = "preserve_order")]
    {
        let _: IndexMap<String, Value> = map.map;
    }
}

