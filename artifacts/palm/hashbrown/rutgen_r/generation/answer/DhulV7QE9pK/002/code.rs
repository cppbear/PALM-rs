// Answer 0

#[test]
fn test_insert_new_key_value() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    assert_eq!(map.insert(1, "value1"), None);
    assert_eq!(map.is_empty(), false);
}

#[test]
fn test_insert_existing_key_update_value() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(1, "value1");
    assert_eq!(map.insert(1, "value2"), Some("value1"));
    assert_eq!(map[&1], "value2");
}

#[test]
fn test_insert_existing_key_update_value_again() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(1, "value1");
    map.insert(1, "value2");
    assert_eq!(map.insert(1, "value3"), Some("value2"));
    assert_eq!(map[&1], "value3");
}

#[test]
fn test_insert_multiple_keys() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    assert_eq!(map.insert(1, "value1"), None);
    assert_eq!(map.insert(2, "value2"), None);
    assert_eq!(map[&1], "value1");
    assert_eq!(map[&2], "value2");
}

#[test]
fn test_insert_key_with_different_value() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(1, "value1");
    assert_eq!(map.insert(1, "value2"), Some("value1"));
    assert_eq!(map[&1], "value2");
    assert_eq!(map.insert(2, "value3"), None);
}

