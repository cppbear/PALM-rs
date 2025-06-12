// Answer 0

#[test]
fn test_swap_remove_entry_existing_key() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    map.insert("key3", "value3");

    let result = map.swap_remove_entry("key2");
    assert_eq!(result, Some(("key2", "value2")));
    assert_eq!(map.len(), 2);
    assert!(!map.contains_key("key2"));
}

#[test]
fn test_swap_remove_entry_non_existing_key() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");

    let result = map.swap_remove_entry("key3");
    assert_eq!(result, None);
    assert_eq!(map.len(), 2);
}

#[test]
fn test_swap_remove_entry_last_element() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("key1", "value1");

    let result = map.swap_remove_entry("key1");
    assert_eq!(result, Some(("key1", "value1")));
    assert_eq!(map.len(), 0);
    assert!(!map.contains_key("key1"));
}

#[test]
fn test_swap_remove_entry_empty_map() {
    use indexmap::IndexMap;

    let mut map: IndexMap<&str, &str> = IndexMap::new();

    let result = map.swap_remove_entry("key1");
    assert_eq!(result, None);
    assert_eq!(map.len(), 0);
}

