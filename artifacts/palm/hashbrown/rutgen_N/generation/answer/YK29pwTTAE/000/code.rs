// Answer 0

#[test]
fn test_contains_key_found() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(1, "a");
    assert_eq!(map.contains_key(&1), true);
}

#[test]
fn test_contains_key_not_found() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(1, "a");
    assert_eq!(map.contains_key(&2), false);
}

#[test]
fn test_contains_key_empty_map() {
    use hashbrown::HashMap;

    let map: HashMap<i32, &str> = HashMap::new();
    assert_eq!(map.contains_key(&1), false);
}

#[test]
fn test_contains_key_with_different_key_type() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert("key", 42);
    assert_eq!(map.contains_key(&"key"), true);
    assert_eq!(map.contains_key(&"not_found"), false);
}

