// Answer 0

#[test]
fn test_append_with_non_empty_other() {
    let mut map1 = Map::new();
    let mut map2 = Map::new();

    map1.insert("key1".to_string(), Value::String("value1".to_string()));
    map1.insert("key2".to_string(), Value::String("value2".to_string()));
    map2.insert("key3".to_string(), Value::String("value3".to_string()));

    map1.append(&mut map2);

    assert_eq!(map1.len(), 3);
    assert!(map1.contains_key("key1"));
    assert!(map1.contains_key("key2"));
    assert!(map1.contains_key("key3"));
    assert_eq!(map2.len(), 0); // map2 should be empty after append
}

#[test]
fn test_append_with_empty_other() {
    let mut map1 = Map::new();
    let mut map2 = Map::new();

    map1.insert("key1".to_string(), Value::String("value1".to_string()));

    map1.append(&mut map2);

    assert_eq!(map1.len(), 1);
    assert!(map1.contains_key("key1"));
    assert_eq!(map2.len(), 0); // map2 should still be empty
}

#[test]
fn test_append_some_keys_multiple_times() {
    let mut map1 = Map::with_capacity(5);
    let mut map2 = Map::with_capacity(5);

    map1.insert("key1".to_string(), Value::String("value1".to_string()));
    map2.insert("key2".to_string(), Value::String("value2".to_string()));
    map2.insert("key3".to_string(), Value::String("value3".to_string()));
    map2.insert("key4".to_string(), Value::String("value4".to_string()));

    map1.append(&mut map2);
    assert_eq!(map1.len(), 4);
    assert!(map1.contains_key("key1"));
    assert!(map1.contains_key("key2"));
    assert!(map1.contains_key("key3"));
    assert!(map1.contains_key("key4"));
    assert_eq!(map2.len(), 0); // map2 should be empty
    
    // Appending again with empty map2
    map1.append(&mut map2);
    assert_eq!(map1.len(), 4); // Length should not change
}

#[test]
fn test_append_into_empty_map() {
    let mut map1 = Map::new();
    let mut map2 = Map::new();
    
    map2.insert("key1".to_string(), Value::String("value1".to_string()));
    map2.insert("key2".to_string(), Value::String("value2".to_string()));

    map1.append(&mut map2);
    
    assert_eq!(map1.len(), 2);
    assert!(map1.contains_key("key1"));
    assert!(map1.contains_key("key2"));
    assert_eq!(map2.len(), 0); // map2 should be empty after append
}

#[should_panic]
#[test]
fn test_append_when_self_is_empty_and_other_is_empty() {
    let mut map1 = Map::new();
    let mut map2 = Map::new();
    
    map1.append(&mut map2); // No panic expected, should just remain empty
    assert_eq!(map1.len(), 0);
}

