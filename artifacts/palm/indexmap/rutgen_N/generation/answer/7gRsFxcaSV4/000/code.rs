// Answer 0

#[test]
fn test_first_entry_empty_map() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, i32> = IndexMap::new();
    let entry = map.first_entry();
    assert_eq!(entry, None);
}

#[test]
fn test_first_entry_single_element() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, i32> = IndexMap::new();
    map.insert(1, 100);
    let entry = map.first_entry();
    assert!(entry.is_some());
    let (key, value) = entry.unwrap().key_value();
    assert_eq!(*key, 1);
    assert_eq!(*value, 100);
}

#[test]
fn test_first_entry_multiple_elements() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, i32> = IndexMap::new();
    map.insert(1, 100);
    map.insert(2, 200);
    let entry = map.first_entry();
    assert!(entry.is_some());
    let (key, value) = entry.unwrap().key_value();
    assert_eq!(*key, 1);
    assert_eq!(*value, 100);
}

