// Answer 0

#[test]
fn test_get_index_mut_valid_index() {
    use indexmap::IndexMap;

    let mut map: IndexMap<u32, String> = IndexMap::new();
    map.insert(0, "zero".to_string());
    map.insert(1, "one".to_string());

    let result = map.get_index_mut(0);
    assert!(result.is_some());
    if let Some((key, value)) = result {
        assert_eq!(*key, 0);
        *value = "modified_zero".to_string();
    }
    assert_eq!(map.get(&0), Some(&"modified_zero".to_string()));
}

#[test]
fn test_get_index_mut_boundary_index() {
    use indexmap::IndexMap;

    let mut map: IndexMap<u32, String> = IndexMap::new();
    map.insert(0, "zero".to_string());
    
    let result = map.get_index_mut(0);
    assert!(result.is_some());
    if let Some((key, value)) = result {
        assert_eq!(*key, 0);
        *value = "modified_zero".to_string();
    }
    assert_eq!(map.get(&0), Some(&"modified_zero".to_string()));

    let result_out_of_bounds = map.get_index_mut(1);
    assert!(result_out_of_bounds.is_none());
}

#[should_panic]
fn test_get_index_mut_invalid_index() {
    use indexmap::IndexMap;

    let mut map: IndexMap<u32, String> = IndexMap::new();
    map.insert(0, "zero".to_string());
    let _ = map.get_index_mut(5);  // This should panic
}

