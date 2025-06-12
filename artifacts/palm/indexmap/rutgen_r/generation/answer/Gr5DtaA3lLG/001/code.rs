// Answer 0

#[test]
fn test_into_keys_with_empty_map() {
    use indexmap::IndexMap;

    let map: IndexMap<i32, i32> = IndexMap::new();
    let keys: Vec<i32> = map.into_keys().collect();
    assert_eq!(keys, Vec::<i32>::new());
}

#[test]
fn test_into_keys_with_single_element() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert(1, 10);
    let keys: Vec<i32> = map.into_keys().collect();
    assert_eq!(keys, vec![1]);
}

#[test]
fn test_into_keys_with_multiple_elements() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    let keys: Vec<i32> = map.into_keys().collect();
    assert_eq!(keys, vec![1, 2, 3]);
}

#[test]
fn test_into_keys_with_duplicate_keys() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert(1, 10);
    map.insert(1, 20); // This will replace the previous value
    let keys: Vec<i32> = map.into_keys().collect();
    assert_eq!(keys, vec![1]);
}

#[test]
fn test_into_keys_with_non_integer_keys() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    let keys: Vec<&str> = map.into_keys().collect();
    assert_eq!(keys, vec!["a", "b"]);
}

