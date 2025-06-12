// Answer 0

#[test]
fn test_first_with_non_empty_map() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    
    let first_entry = map.first();
    assert_eq!(first_entry, Some((&"a", &1)));
}

#[test]
fn test_first_with_empty_map() {
    use indexmap::IndexMap;

    let map: IndexMap<&str, i32> = IndexMap::new();
    
    let first_entry = map.first();
    assert_eq!(first_entry, None);
}

