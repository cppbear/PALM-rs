// Answer 0

#[test]
fn test_get_existing_key() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(1, "a");
    assert_eq!(map.get(&1), Some(&"a"));
}

#[test]
fn test_get_non_existing_key() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(1, "a");
    assert_eq!(map.get(&2), None);
}

#[test]
fn test_get_with_different_borrowed_type() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(1, "a");
    let key: &i32 = &1;
    assert_eq!(map.get(key), Some(&"a"));
}

#[test]
fn test_get_empty_map() {
    use hashbrown::HashMap;

    let map: HashMap<i32, &str> = HashMap::new();
    assert_eq!(map.get(&1), None);
}

