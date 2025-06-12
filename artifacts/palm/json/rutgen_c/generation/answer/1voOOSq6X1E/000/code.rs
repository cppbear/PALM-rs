// Answer 0

#[test]
fn test_append_empty_maps() {
    let mut map1 = Map::new();
    let mut map2 = Map::new();
    map1.append(&mut map2);
    assert!(map1.is_empty());
    assert!(map2.is_empty());
}

#[test]
fn test_append_non_empty_map_to_empty_map() {
    let mut map1 = Map::new();
    let mut map2 = Map::new();
    map2.insert("key1".to_string(), Value::Bool(true));
    map1.append(&mut map2);
    assert_eq!(map1.len(), 1);
    assert_eq!(map2.len(), 0);
    assert_eq!(map1.get("key1"), Some(&Value::Bool(true)));
}

#[test]
fn test_append_empty_map_to_non_empty_map() {
    let mut map1 = Map::new();
    map1.insert("key1".to_string(), Value::Bool(true));
    let mut map2 = Map::new();
    map1.append(&mut map2);
    assert_eq!(map1.len(), 1);
    assert_eq!(map2.len(), 0);
    assert_eq!(map1.get("key1"), Some(&Value::Bool(true)));
}

#[test]
fn test_append_non_empty_maps() {
    let mut map1 = Map::new();
    map1.insert("key1".to_string(), Value::Bool(true));
    let mut map2 = Map::new();
    map2.insert("key2".to_string(), Value::Number(Number::from(42)));
    map1.append(&mut map2);
    assert_eq!(map1.len(), 2);
    assert_eq!(map2.len(), 0);
    assert_eq!(map1.get("key1"), Some(&Value::Bool(true)));
    assert_eq!(map1.get("key2"), Some(&Value::Number(Number::from(42))));
}

