// Answer 0

#[test]
fn test_remove_existing_key() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(1, "a");
    map.insert(2, "b");
    
    assert_eq!(map.remove(&1), Some("a"));
    assert_eq!(map.remove(&2), Some("b"));
    assert!(map.is_empty());
}

#[test]
fn test_remove_non_existing_key() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(1, "a");
    
    assert_eq!(map.remove(&2), None);
    assert_eq!(map.remove(&3), None);
    assert_eq!(map.len(), 1);
}

#[test]
fn test_remove_multiple_same_keys() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(1, "a");
    map.insert(1, "b");
    
    assert_eq!(map.remove(&1), Some("b")); // Last inserted value
    assert_eq!(map.remove(&1), None);
    assert!(map.is_empty());
}

#[test]
fn test_remove_and_check_capacity() {
    use hashbrown::HashMap;
    
    let mut map = HashMap::with_capacity(2);
    assert!(map.capacity() >= 2);

    map.insert(1, "a");
    assert_eq!(map.remove(&1), Some("a"));
    assert!(map.is_empty());
}

