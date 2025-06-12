// Answer 0

#[test]
fn test_last_entry_non_empty_map() {
    use indexmap::IndexMap;

    let mut map: IndexMap<u32, String> = IndexMap::new();
    map.insert(1, "First".to_string());
    map.insert(2, "Second".to_string());
    map.insert(3, "Third".to_string());

    let last_entry = map.last_entry();
    assert!(last_entry.is_some());
    let entry = last_entry.unwrap();
    assert_eq!(entry.key(), &3);
    assert_eq!(entry.get(), "Third");
}

#[test]
#[should_panic]
fn test_last_entry_empty_map() {
    use indexmap::IndexMap;

    let mut map: IndexMap<u32, String> = IndexMap::new();
    let _last_entry = map.last_entry(); // Should trigger a panic as map is empty
}

