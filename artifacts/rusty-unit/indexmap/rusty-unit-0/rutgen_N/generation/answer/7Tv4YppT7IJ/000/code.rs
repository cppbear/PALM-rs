// Answer 0

#[test]
fn test_keys_empty_map() {
    use indexmap::IndexMap;

    let map: IndexMap<i32, i32> = IndexMap::new();
    let keys: Vec<_> = map.keys().collect();
    assert_eq!(keys, Vec::<i32>::new());
}

#[test]
fn test_keys_single_entry() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert(1, 100);
    let keys: Vec<_> = map.keys().collect();
    assert_eq!(keys, vec![&1]);
}

#[test]
fn test_keys_multiple_entries() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert(1, 100);
    map.insert(2, 200);
    map.insert(3, 300);
    let keys: Vec<_> = map.keys().collect();
    assert_eq!(keys, vec![&1, &2, &3]);
}

#[test]
fn test_keys_ordered_entries() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert(2, 200);
    map.insert(1, 100);
    map.insert(3, 300);
    let keys: Vec<_> = map.keys().collect();
    assert_eq!(keys, vec![&2, &1, &3]);
}

