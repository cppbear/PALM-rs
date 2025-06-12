// Answer 0

#[test]
fn test_remove_entry_existing_key() {
    use std::hash::Hash;
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("key1", 10);
    map.insert("key2", 20);

    let result = map.remove_entry("key1");

    assert_eq!(result, Some(("key1", 10)));
    assert_eq!(map.len(), 1);
    assert!(!map.contains_key("key1"));
}

#[test]
fn test_remove_entry_non_existing_key() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("key1", 10);
    map.insert("key2", 20);

    let result = map.remove_entry("key3");

    assert_eq!(result, None);
    assert_eq!(map.len(), 2);
}

#[test]
fn test_remove_entry_key_with_multiple_entries() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("key1", 10);
    map.insert("key1", 20); // Demonstrating handling of duplicate keys

    let result = map.remove_entry("key1");

    assert_eq!(result, Some(("key1", 20))); // Remove the last inserted with the same key
    assert_eq!(map.len(), 0);
}

