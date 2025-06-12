// Answer 0

#[test]
fn test_is_empty_when_map_is_empty() {
    use hashbrown::HashMap;

    let map: HashMap<i32, &str> = HashMap::new();
    assert!(map.is_empty());
}

#[test]
fn test_is_empty_when_map_has_one_element() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(1, "a");
    assert!(!map.is_empty());
}

#[test]
fn test_is_empty_when_map_has_multiple_elements() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(1, "a");
    map.insert(2, "b");
    assert!(!map.is_empty());
}

#[test]
fn test_is_empty_after_removal_of_last_element() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(1, "a");
    map.remove(&1);
    assert!(map.is_empty());
}

#[test]
fn test_is_empty_on_map_with_capacity() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, &str> = HashMap::with_capacity(10);
    assert!(map.is_empty());
}

