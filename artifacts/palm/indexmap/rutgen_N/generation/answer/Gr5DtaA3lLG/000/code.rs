// Answer 0

#[test]
fn test_into_keys_empty_map() {
    use indexmap::IndexMap;

    let map: IndexMap<i32, i32> = IndexMap::new();
    let keys: Vec<i32> = map.into_keys().collect();
    assert_eq!(keys, Vec::<i32>::new());
}

#[test]
fn test_into_keys_single_entry() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, i32> = IndexMap::new();
    map.insert(1, 10);
    let keys: Vec<i32> = map.into_keys().collect();
    assert_eq!(keys, vec![1]);
}

#[test]
fn test_into_keys_multiple_entries() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, i32> = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    let keys: Vec<i32> = map.into_keys().collect();
    assert_eq!(keys, vec![1, 2, 3]);
}

#[test]
fn test_into_keys_ordering() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, i32> = IndexMap::new();
    map.insert(3, 30);
    map.insert(1, 10);
    map.insert(2, 20);
    let keys: Vec<i32> = map.into_keys().collect();
    assert_eq!(keys, vec![3, 1, 2]);
}

