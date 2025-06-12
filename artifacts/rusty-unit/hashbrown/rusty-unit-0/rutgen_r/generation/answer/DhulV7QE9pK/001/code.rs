// Answer 0

#[test]
fn test_insert_new_key() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    let insert_result = map.insert(1, "value1");
    assert_eq!(insert_result, None);
    assert_eq!(map.len(), 1);
}

#[test]
fn test_insert_existing_key() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(1, "value1");
    let old_value = map.insert(1, "value2");
    assert_eq!(old_value, Some("value1"));
    assert_eq!(map.len(), 1);
    assert_eq!(map.get(&1), Some(&"value2"));
}

#[test]
fn test_insert_multiple_keys() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(2, "value2");
    map.insert(3, "value3");
    let insert_result = map.insert(4, "value4");
    assert_eq!(insert_result, None);
    assert_eq!(map.len(), 3);
}

#[test]
fn test_insert_with_different_types() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, &str> = HashMap::new();
    let insert_result = map.insert(100, "test");
    assert_eq!(insert_result, None);
    assert_eq!(map.get(&100), Some(&"test"));
}

