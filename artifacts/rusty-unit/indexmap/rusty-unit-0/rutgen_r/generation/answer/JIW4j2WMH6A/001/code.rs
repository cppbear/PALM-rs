// Answer 0

#[test]
fn test_shift_remove_existing_key() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert(1, "a");
    map.insert(2, "b");
    map.insert(3, "c");

    let result = map.shift_remove(&2);
    assert_eq!(result, Some("b"));
    assert_eq!(map.len(), 2);
    assert_eq!(map.get(&1), Some(&"a"));
    assert_eq!(map.get(&3), Some(&"c"));
}

#[test]
fn test_shift_remove_non_existing_key() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert(1, "a");
    map.insert(2, "b");

    let result = map.shift_remove(&3);
    assert_eq!(result, None);
    assert_eq!(map.len(), 2);
    assert_eq!(map.get(&1), Some(&"a"));
    assert_eq!(map.get(&2), Some(&"b"));
}

#[test]
fn test_shift_remove_multiple_elements() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert(1, "a");
    map.insert(2, "b");
    map.insert(3, "c");
    map.insert(4, "d");

    let result = map.shift_remove(&2);
    assert_eq!(result, Some("b"));
    assert_eq!(map.len(), 3);
    assert_eq!(map.get(&1), Some(&"a"));
    assert_eq!(map.get(&3), Some(&"c"));
    assert_eq!(map.get(&4), Some(&"d"));
}

#[test]
fn test_shift_remove_first_element() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert(1, "a");
    map.insert(2, "b");
    map.insert(3, "c");

    let result = map.shift_remove(&1);
    assert_eq!(result, Some("a"));
    assert_eq!(map.len(), 2);
    assert_eq!(map.get(&2), Some(&"b"));
    assert_eq!(map.get(&3), Some(&"c"));
}

#[test]
fn test_shift_remove_last_element() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert(1, "a");
    map.insert(2, "b");
    map.insert(3, "c");

    let result = map.shift_remove(&3);
    assert_eq!(result, Some("c"));
    assert_eq!(map.len(), 2);
    assert_eq!(map.get(&1), Some(&"a"));
    assert_eq!(map.get(&2), Some(&"b"));
}

