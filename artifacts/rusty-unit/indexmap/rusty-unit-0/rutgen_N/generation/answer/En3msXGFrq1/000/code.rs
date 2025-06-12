// Answer 0

#[test]
fn test_remove_existing_key() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert(1, "value1");
    map.insert(2, "value2");

    let value = map.remove(&1);
    assert_eq!(value, Some("value1"));
    assert_eq!(map.len(), 1);
    assert!(map.get(&1).is_none());
}

#[test]
fn test_remove_non_existing_key() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert(1, "value1");
    
    let value = map.remove(&3);
    assert_eq!(value, None);
    assert_eq!(map.len(), 1);
}

#[test]
fn test_remove_multiple_keys() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert(1, "value1");
    map.insert(2, "value2");
    map.insert(3, "value3");

    assert_eq!(map.remove(&2), Some("value2"));
    assert_eq!(map.len(), 2);
    assert!(map.get(&2).is_none());
    
    assert_eq!(map.remove(&1), Some("value1"));
    assert_eq!(map.len(), 1);
    
    assert_eq!(map.remove(&3), Some("value3"));
    assert_eq!(map.len(), 0);
}

#[test]
fn test_remove_from_empty_map() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    
    let value = map.remove(&1);
    assert_eq!(value, None);
    assert_eq!(map.len(), 0);
}

#[test]
fn test_remove_with_different_type_key() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("key1", 1);
    map.insert("key2", 2);

    let value = map.remove(&"key1");
    assert_eq!(value, Some(1));
    assert_eq!(map.len(), 1);
    assert!(map.get(&"key1").is_none());
}

