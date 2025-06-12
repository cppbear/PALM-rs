// Answer 0

#[test]
fn test_first_entry_non_empty_map() {
    use indexmap::IndexMap;

    let mut map: IndexMap<&str, i32> = IndexMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    
    let first_entry = map.first_entry().unwrap();
    assert_eq!(first_entry.key(), &"a");
    assert_eq!(first_entry.get(), &1);
}

#[test]
fn test_first_entry_empty_map() {
    use indexmap::IndexMap;

    let mut map: IndexMap<&str, i32> = IndexMap::new();
    
    let first_entry = map.first_entry();
    assert!(first_entry.is_none());
}

#[test]
#[should_panic]
fn test_first_entry_panic_condition() {
    use indexmap::IndexMap;

    let mut map: IndexMap<&str, i32> = IndexMap::new();
    
    // Attempt to manipulate the first entry without checking if it exists
    let _entry = map.first_entry().unwrap(); // This should panic
}

