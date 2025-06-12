// Answer 0

#[test]
fn test_remove_entry_existing_key() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("key1", 1);
    map.insert("key2", 2);
    map.insert("key3", 3);

    let result = map.remove_entry("key2");
    assert_eq!(result, Some(("key2", 2)));
    assert_eq!(map.len(), 2);
}

#[test]
fn test_remove_entry_non_existing_key() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("key1", 1);
    map.insert("key2", 2);

    let result = map.remove_entry("key3");
    assert_eq!(result, None);
    assert_eq!(map.len(), 2);
}

#[test]
#[should_panic]
fn test_remove_entry_panic_with_invalid_key_type() {
    use indexmap::IndexMap;

    struct InvalidKey;
    
    let mut map: IndexMap<_, _> = IndexMap::new();
    map.insert("key1", 1);

    // Attempt to remove entry with a type that does not implement `Hash` or `Equivalent`
    let _ = map.remove_entry(&InvalidKey);
}

#[test]
fn test_remove_entry_multiple_removals() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("key1", 1);
    map.insert("key2", 2);
    map.insert("key3", 3);

    let result1 = map.remove_entry("key1");
    assert_eq!(result1, Some(("key1", 1)));
    
    let result2 = map.remove_entry("key2");
    assert_eq!(result2, Some(("key2", 2)));

    assert_eq!(map.len(), 1);
    assert_eq!(map.remove_entry("key3"), Some(("key3", 3)));
    assert_eq!(map.len(), 0);
}

#[test]
fn test_remove_entry_empty_map() {
    use indexmap::IndexMap;

    let mut map: IndexMap<&str, i32> = IndexMap::new();
    let result = map.remove_entry("key1");
    assert_eq!(result, None);
}

