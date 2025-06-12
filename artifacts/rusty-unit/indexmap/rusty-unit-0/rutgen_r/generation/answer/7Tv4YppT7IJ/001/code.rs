// Answer 0

#[test]
fn test_keys_empty_map() {
    use indexmap::IndexMap;

    let map: IndexMap<i32, &str> = IndexMap::new();
    let keys: Vec<i32> = map.keys().cloned().collect();
    assert!(keys.is_empty());
}

#[test]
fn test_keys_non_empty_map() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, &str> = IndexMap::new();
    map.insert(1, "one");
    map.insert(2, "two");
    map.insert(3, "three");

    let keys: Vec<i32> = map.keys().cloned().collect();
    assert_eq!(keys, vec![1, 2, 3]);
}

#[test]
fn test_keys_order_of_insertion() {
    use indexmap::IndexMap;

    let mut map: IndexMap<&str, i32> = IndexMap::new();
    map.insert("b", 2);
    map.insert("a", 1);
    map.insert("c", 3);

    let keys: Vec<&str> = map.keys().cloned().collect();
    assert_eq!(keys, vec!["b", "a", "c"]);
}

