// Answer 0

#[test]
fn test_swap_remove_full_empty_map() {
    use indexmap::IndexMap;

    let mut map: IndexMap<String, i32> = IndexMap::new();
    let result = map.swap_remove_full(&"test_key");
    
    assert_eq!(result, None);
}

#[test]
fn test_swap_remove_full_single_entry_no_match() {
    use indexmap::IndexMap;

    let mut map: IndexMap<String, i32> = IndexMap::new();
    map.insert("other_key".to_string(), 42);
    
    let result = map.swap_remove_full(&"test_key");
    
    assert_eq!(result, None);
}

#[test]
fn test_swap_remove_full_single_entry_match() {
    use indexmap::IndexMap;

    let mut map: IndexMap<String, i32> = IndexMap::new();
    map.insert("test_key".to_string(), 42);
    
    let result = map.swap_remove_full(&"test_key");
    
    assert_eq!(result, Some((0, "test_key".to_string(), 42)));
}

