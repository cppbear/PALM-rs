// Answer 0

#[test]
fn test_get_key_value_existing_key() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(1, "a");
    assert_eq!(map.get_key_value(&1), Some((&1, &"a")));
}

#[test]
fn test_get_key_value_non_existing_key() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(1, "a");
    assert_eq!(map.get_key_value(&2), None);
}

#[test]
fn test_get_key_value_with_different_borrowed_form() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(1, "a");
    let key: &dyn std::hash::Hash = &1; // Using a trait object for borrowed form
    assert_eq!(map.get_key_value(key), Some((&1, &"a")));
}

#[test]
fn test_get_key_value_on_empty_map() {
    use hashbrown::HashMap;

    let map: HashMap<i32, &str> = HashMap::new();
    assert_eq!(map.get_key_value(&1), None);
}

