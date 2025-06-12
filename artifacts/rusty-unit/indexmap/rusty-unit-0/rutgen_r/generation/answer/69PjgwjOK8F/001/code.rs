// Answer 0

#[test]
fn test_into_values_non_empty_map() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, &str> = IndexMap::new();
    map.insert(1, "one");
    map.insert(2, "two");
    map.insert(3, "three");

    let values: Vec<&str> = Box::new(map).into_values().collect();
    assert_eq!(values, vec!["one", "two", "three"]);
}

#[test]
fn test_into_values_empty_map() {
    use indexmap::IndexMap;

    let map: IndexMap<i32, &str> = IndexMap::new();

    let values: Vec<&str> = Box::new(map).into_values().collect();
    assert_eq!(values.len(), 0);
}

#[test]
#[should_panic]
fn test_into_values_panic_on_exceeding_slice() {
    use indexmap::IndexMap;

    let map: IndexMap<i32, &str> = IndexMap::new();
    let values: Vec<&str> = Box::new(map).into_values().collect();

    // Attempting to call a method on an empty values iterator should not panic.
    let _ = values[0];
}

