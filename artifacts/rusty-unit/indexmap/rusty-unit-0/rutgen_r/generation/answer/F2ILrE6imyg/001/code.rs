// Answer 0

#[test]
fn test_values_non_empty_map() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, &str> = IndexMap::new();
    map.insert(1, "one");
    map.insert(2, "two");
    map.insert(3, "three");

    let values: Vec<_> = map.values().cloned().collect();
    assert_eq!(values, vec!["one", "two", "three"]);
}

#[test]
fn test_values_empty_map() {
    use indexmap::IndexMap;

    let map: IndexMap<i32, &str> = IndexMap::new();
    let values: Vec<_> = map.values().cloned().collect();
    assert_eq!(values, Vec::<&str>::new());
}

#[test]
fn test_values_single_entry_map() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, &str> = IndexMap::new();
    map.insert(42, "forty-two");

    let values: Vec<_> = map.values().cloned().collect();
    assert_eq!(values, vec!["forty-two"]);
}

#[test]
fn test_values_order_of_entries() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, &str> = IndexMap::new();
    map.insert(10, "ten");
    map.insert(20, "twenty");
    map.insert(30, "thirty");

    let values: Vec<_> = map.values().cloned().collect();
    assert_eq!(values, vec!["ten", "twenty", "thirty"]);
}

