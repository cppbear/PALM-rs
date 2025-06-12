// Answer 0

#[test]
fn test_last_mut_non_empty() {
    use indexmap::IndexMap; // Ensure to use the correct import for indexmap
    let mut map = IndexMap::new();
    map.insert("key1", 10);
    map.insert("key2", 20);

    if let Some((key, value)) = map.last_mut() {
        assert_eq!(*key, "key2");
        *value += 5; // Mutate the value
        assert_eq!(map["key2"], 25);
    } else {
        panic!("Expected last_mut to return Some value for non-empty map.");
    }
}

#[test]
fn test_last_mut_empty() {
    use indexmap::IndexMap; // Ensure to use the correct import for indexmap
    let mut map: IndexMap<&str, i32> = IndexMap::new();

    // This should return None and not panic
    assert!(map.last_mut().is_none());
}

#[test]
#[should_panic]
fn test_last_mut_panic() {
    use indexmap::IndexMap; // Ensure to use the correct import for indexmap
    let mut map = IndexMap::new();
    // Intentionally calling last_mut on a mutable reference to a still empty map
    map.last_mut().unwrap(); // This should panic
}

