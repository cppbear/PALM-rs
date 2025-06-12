// Answer 0

#[test]
fn test_values_empty_map() {
    use indexmap::IndexMap;

    let map: IndexMap<i32, i32> = IndexMap::new();
    let values: Vec<i32> = map.values().cloned().collect();
    assert!(values.is_empty());
}

#[test]
fn test_values_single_element_map() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, i32> = IndexMap::new();
    map.insert(1, 10);
    let values: Vec<i32> = map.values().cloned().collect();
    assert_eq!(values, vec![10]);
}

#[test]
fn test_values_multiple_elements_map() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, i32> = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    let values: Vec<i32> = map.values().cloned().collect();
    assert_eq!(values, vec![10, 20, 30]);
}

#[test]
fn test_values_ordering_in_map() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, i32> = IndexMap::new();
    map.insert(2, 20);
    map.insert(1, 10);
    map.insert(3, 30);
    let values: Vec<i32> = map.values().cloned().collect();
    assert_eq!(values, vec![20, 10, 30]);
}

