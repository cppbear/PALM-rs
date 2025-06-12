// Answer 0

#[test]
fn test_last_entry_empty_map() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, i32> = IndexMap::new();
    assert_eq!(map.last_entry(), None);
}

#[test]
fn test_last_entry_single_entry() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, i32> = IndexMap::new();
    map.insert(1, 10);
    let entry = map.last_entry();
    assert!(entry.is_some());
    assert_eq!(entry.unwrap().key(), &1);
    assert_eq!(entry.unwrap().get(), &10);
}

#[test]
fn test_last_entry_multiple_entries() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, i32> = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    
    let entry = map.last_entry();
    assert!(entry.is_some());
    assert_eq!(entry.unwrap().key(), &3);
    assert_eq!(entry.unwrap().get(), &30);
}

