// Answer 0

#[test]
fn test_shift_remove_existing_key() {
    use indexmap::IndexMap;
    use std::hash::Hash;

    let mut map: IndexMap<i32, &str> = IndexMap::new();
    map.insert(1, "one");
    map.insert(2, "two");
    map.insert(3, "three");

    let removed_value = map.shift_remove(&2);
    assert_eq!(removed_value, Some("two"));
    assert!(map.get(&2).is_none());
    assert_eq!(map.get(&1), Some(&"one"));
    assert_eq!(map.get(&3), Some(&"three"));
}

#[test]
fn test_shift_remove_non_existent_key() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, &str> = IndexMap::new();
    map.insert(1, "one");
    map.insert(2, "two");

    let removed_value = map.shift_remove(&3);
    assert_eq!(removed_value, None);
    assert_eq!(map.len(), 2);
}

#[test]
fn test_shift_remove_multiple_removals() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, &str> = IndexMap::new();
    map.insert(1, "one");
    map.insert(2, "two");
    map.insert(3, "three");

    assert_eq!(map.shift_remove(&1), Some("one"));
    assert_eq!(map.shift_remove(&3), Some("three"));
    assert_eq!(map.len(), 1);
    assert_eq!(map.get(&2), Some(&"two"));
}

#[test]
fn test_shift_remove_preserves_order() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, &str> = IndexMap::new();
    map.insert(1, "one");
    map.insert(2, "two");
    map.insert(3, "three");
    map.insert(4, "four");

    let removed_value = map.shift_remove(&2);
    assert_eq!(removed_value, Some("two"));

    assert_eq!(map.get_index(0), Some((&1, &"one")));
    assert_eq!(map.get_index(1), Some((&3, &"three")));
    assert_eq!(map.get_index(2), Some((&4, &"four")));
}

