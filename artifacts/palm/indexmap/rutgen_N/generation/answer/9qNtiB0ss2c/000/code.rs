// Answer 0

#[test]
fn test_last_entry_non_empty_map() {
    use indexmap::IndexMap;
    let mut map = IndexMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");

    if let Some(entry) = map.last_entry() {
        assert_eq!(entry.key(), "key2");
        assert_eq!(entry.value(), "value2");
    } else {
        panic!("Expected Some entry for non-empty map");
    }
}

#[test]
fn test_last_entry_empty_map() {
    use indexmap::IndexMap;
    let mut map: IndexMap<&str, &str> = IndexMap::new();

    let entry = map.last_entry();
    assert!(entry.is_none(), "Expected None for empty map");
}

#[test]
fn test_last_entry_single_element_map() {
    use indexmap::IndexMap;
    let mut map = IndexMap::new();
    map.insert("key1", "value1");

    if let Some(entry) = map.last_entry() {
        assert_eq!(entry.key(), "key1");
        assert_eq!(entry.value(), "value1");
    } else {
        panic!("Expected Some entry for single element map");
    }
}

