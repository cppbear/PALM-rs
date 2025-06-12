// Answer 0

#[test]
fn test_get_index_entry_valid_index() {
    use indexmap::IndexMap; // Ensure you replace this with the actual import path for your crate
    use indexmap::map::IndexedEntry;

    let mut map = IndexMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);

    let entry = map.get_index_entry(1);
    assert!(entry.is_some());
    if let Some(IndexedEntry { key, value }) = entry {
        assert_eq!(key, "b");
        assert_eq!(*value, 2);
    }
}

#[test]
fn test_get_index_entry_invalid_index_too_high() {
    use indexmap::IndexMap; // Ensure you replace this with the actual import path for your crate

    let mut map = IndexMap::new();
    map.insert("a", 1);
    map.insert("b", 2);

    let entry = map.get_index_entry(2);
    assert!(entry.is_none());
}

#[test]
fn test_get_index_entry_invalid_index_negative() {
    use indexmap::IndexMap; // Ensure you replace this with the actual import path for your crate

    let mut map = IndexMap::new();
    map.insert("a", 1);
    map.insert("b", 2);

    let entry = map.get_index_entry(usize::MAX);
    assert!(entry.is_none());
}

#[test]
fn test_get_index_entry_empty_map() {
    use indexmap::IndexMap; // Ensure you replace this with the actual import path for your crate

    let mut map = IndexMap::new();
    let entry = map.get_index_entry(0);
    assert!(entry.is_none());
}

