// Answer 0

#[test]
fn test_into_values_with_non_empty_map() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, &str> = IndexMap::new();
    map.insert(1, "one");
    map.insert(2, "two");
    map.insert(3, "three");

    let values: Vec<&str> = map.into_values().collect();
    assert_eq!(values, vec!["one", "two", "three"]);
}

#[test]
fn test_into_values_with_empty_map() {
    use indexmap::IndexMap;

    let map: IndexMap<i32, &str> = IndexMap::new();
    
    let values: Vec<&str> = map.into_values().collect();
    assert_eq!(values, Vec::<&str>::new());
}

