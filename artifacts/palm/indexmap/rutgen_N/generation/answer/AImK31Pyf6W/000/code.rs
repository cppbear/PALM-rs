// Answer 0

#[test]
fn test_shift_remove_full_existing_key() {
    use indexmap::IndexMap;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut map = IndexMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    map.insert("key3", "value3");

    let result = map.shift_remove_full(&"key2");
    assert_eq!(result, Some((1, "key2", "value2")));
    assert_eq!(map.len(), 2);
    assert!(!map.contains_key("key2"));
}

#[test]
fn test_shift_remove_full_non_existing_key() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("key1", "value1");

    let result = map.shift_remove_full(&"non_existing_key");
    assert_eq!(result, None);
    assert_eq!(map.len(), 1);
}

#[test]
fn test_shift_remove_full_only_element() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("only_key", "only_value");

    let result = map.shift_remove_full(&"only_key");
    assert_eq!(result, Some((0, "only_key", "only_value")));
    assert_eq!(map.len(), 0);
}

#[test]
fn test_shift_remove_full_empty_map() {
    use indexmap::IndexMap;

    let mut map: IndexMap<&str, &str> = IndexMap::new();

    let result = map.shift_remove_full(&"any_key");
    assert_eq!(result, None);
    assert_eq!(map.len(), 0);
}

