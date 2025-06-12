// Answer 0

#[test]
fn test_remove_existing_key() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(1, "a");

    assert_eq!(map.remove(&1), Some("a"));
    assert!(map.is_empty());
}

#[test]
fn test_remove_non_existing_key() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    assert_eq!(map.remove(&1), None);
}

#[test]
fn test_remove_key_twice() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(1, "a");

    assert_eq!(map.remove(&1), Some("a"));
    assert_eq!(map.remove(&1), None);
    assert!(map.is_empty());
}

#[test]
fn test_remove_key_with_different_borrowed_form() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(1, "a");

    let key: &i32 = &1;
    assert_eq!(map.remove(key), Some("a"));
    assert!(map.is_empty());
}

#[test]
fn test_remove_multiple_keys() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(1, "a");
    map.insert(2, "b");
    map.insert(3, "c");

    assert_eq!(map.remove(&2), Some("b"));
    assert_eq!(map.remove(&3), Some("c"));
    assert_eq!(map.remove(&1), Some("a"));
    assert!(map.is_empty());
}

